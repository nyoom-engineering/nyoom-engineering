open Printf
module S = Set.Make(String)

(* canonical string for a pitch‑class set and its inversion *)
let canon pcs =
  let sort = List.sort_uniq compare in
  let s   = sort pcs
  and inv = sort (List.map (fun p -> (12 - p) mod 12) pcs) in
  String.concat "," (List.map string_of_int (if s < inv then s else inv))

let add pcs set = S.add (canon pcs) set

(* generate every transposition & inversion (rotations) of each interval pattern *)
let gen patterns =
  let roots  = List.init 12 (fun r -> r) in
  let rot xs i =
    let n = List.length xs in
    List.init n (fun k -> List.nth xs ((i + k) mod n))
  in
  List.concat_map (fun ints ->
    List.concat_map (fun r ->
      List.init (List.length ints) (fun i ->
        List.map (fun x -> (r + x) mod 12) (rot ints i)
      )
    ) roots
  ) patterns

let tri   = gen [[0;4;7]; [0;3;7]; [0;3;6]; [0;4;8]]
let sev   = gen [[0;4;7;11]; [0;4;7;10]; [0;3;7;10]; [0;3;7;11];
                 [0;3;6;10]; [0;3;6;9];  [0;4;8;11]; [0;4;8;10]; [0;3;6;11]]
let universe =
  let rec bits n i acc = if i=12 then acc else bits n (i+1) (if n land (1 lsl i) <> 0 then i::acc else acc) in
  List.init ((1 lsl 12) - 1) (fun n -> bits n 0 [])

let uniq  l = List.fold_left (fun s pcs -> add pcs s) S.empty l |> S.cardinal
let pct t u = 100. *. float_of_int (t - u) /. float_of_int t

let () =
  let utri, usev, uall = uniq tri, uniq sev, uniq universe in
  printf "Triads : %d unique (%.1f%% redundant)\n" utri (pct 144 utri);
  printf "7ths   : %d unique (%.1f%% redundant)\n" usev (pct 432 usev);
  printf "All PC sets (size 1‑11): %d unique (%.1f%% redundant)\n"
         uall (pct 4095 uall)

(* Triads : 22 unique (84.7% redundant)
   7ths   : 58 unique (86.6% redundant)
   All PC sets (size 1‑11): 2111 unique (48.4% redundant) *)

 let rec fact n = if n = 0 then 1 else n * fact (n - 1)