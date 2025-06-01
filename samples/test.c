// file: extension.c
#define PY_SSIZE_T_CLEAN
#include <Python.h>
 
#include <stdio.h>
#define QISKIT_C_PYTHON_INTERFACE
#include <qiskit.h>
 
QkObs *build_observable() {
    // build a 100-qubit empty observable
    u_int32_t num_qubits = 100;
    QkObs *obs = qk_obs_zero(num_qubits);
 
    // add the term 2 * (X0 Y1 Z2) to the observable
    complex double coeff = 2;  // the coefficient
    QkBitTerm bit_terms[3] = {QkBitTerm_X, QkBitTerm_Y, QkBitTerm_Z};  // bit terms: X Y Z
    uint32_t indices[3] = {0, 1, 2};  // indices: 0 1 2
    QkObsTerm term = {coeff, 3, bit_terms, indices, num_qubits};
    qk_obs_add_term(obs, &term);  // append the term
 
    return obs;
}
 
/// Define the Python function, which will internally build the QkObs using the
/// C function defined above, and then convert the C object to the Python equivalent:
/// a SparseObservable, handled as PyObject.
static PyObject *cextension_build_observable(PyObject *self, PyObject *args) {
    // At this point, ``args`` could be parsed for arguments. See PyArg_ParseTuple for details.
    QkObs *obs = build_observable();  // call the C function to build the observable
    PyObject *py_obs = qk_obs_to_python(obs);  // convert QkObs to the Python-equivalent
    return py_obs;
}
 
/// Define the module methods.
static PyMethodDef CExtMethods[] = {
    {"build_observable", cextension_build_observable, METH_VARARGS, "Build an observable."},
    {NULL, NULL, 0, NULL}, // sentinel
};
 
/// Define the module, which here is called ``cextension``.
static struct PyModuleDef cextension = {
    PyModuleDef_HEAD_INIT,
    "cextension", // module name
    NULL,         // docs
    -1,           // keep the module state in global variables
    CExtMethods,
};
 
PyMODINIT_FUNC PyInit_cextension(void) { return PyModule_Create(&cextension); }
 
int main(int argc, char *argv[]) {
    PyStatus status;
    PyConfig config;
    PyConfig_InitPythonConfig(&config);
 
    // Add a built-in module, before Py_Initialize.
    if (PyImport_AppendInittab("cextension", PyInit_cextension) == -1) {
        fprintf(stderr, "Error: could not extend in-built modules table\n");
        exit(1);
    }
 
    // Pass argv[0] to the Python interpreter.
    status = PyConfig_SetBytesString(&config, &config.program_name, argv[0]);
    if (PyStatus_Exception(status)) {
        goto exception;
    }
 
    // Initialize the Python interpreter.
    status = Py_InitializeFromConfig(&config);
    if (PyStatus_Exception(status)) {
        goto exception;
    }
    PyConfig_Clear(&config);
 
    // Import the module.
    PyObject *pmodule = PyImport_ImportModule("cextension");
    if (!pmodule) {
        PyErr_Print();
        fprintf(stderr, "Error: could not import module 'cextension'\n");
    }
 
    return 0;
 
exception:
    PyConfig_Clear(&config);
    Py_ExitStatusException(status);
}

// fast inverse square root
float Q_rsqrt( float number )
{
	long i;
	float x2, y;
	const float threehalfs = 1.5F;

	x2 = number * 0.5F;
	y  = number;
	i  = * ( long * ) &y;                       // evil floating point bit level hacking
	i  = 0x5f3759df - ( i >> 1 );               // what the fuck?
	y  = * ( float * ) &i;
	y  = y * ( threehalfs - ( x2 * y * y ) );   // 1st iteration
//	y  = y * ( threehalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed

	return y;
}