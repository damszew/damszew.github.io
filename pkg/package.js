
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) wasm.__wbindgen_export_2.get(dtor)(a, state.b);
            else state.a = a;
        }
    };
    real.original = state;
    return real;
}

function logError(e) {
    let error = (function () {
        try {
            return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
        } catch(_) {
            return "<failed to stringify thrown value>";
        }
    }());
    console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
    throw e;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}
function __wbg_adapter_20(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0a7303a2513b36c1(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_23(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h43381aa399a6ad0f(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_26(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9cfb06f24abf35f0(arg0, arg1, arg2);
}

/**
*/
export function render() {
    wasm.render();
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}
function __wbg_adapter_200(arg0, arg1, arg2, arg3, arg4) {
    _assertNum(arg0);
    _assertNum(arg1);
    _assertNum(arg3);
    wasm.wasm_bindgen__convert__closures__invoke3_mut__h08bfe33b01cf8458(arg0, arg1, addHeapObject(arg2), arg3, addHeapObject(arg4));
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_cb_forget = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            try {
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(arg0, arg1);
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        try {
            var ret = new Error();
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).stack;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_Window_04bba8b54ef81db0 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof Window;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_document_f023a2b0d5b3d060 = function(arg0) {
        try {
            var ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_location_71ee6c222f3effa4 = function(arg0) {
        try {
            var ret = getObject(arg0).location;
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_history_1189c3a9e284c8c1 = function(arg0) {
        try {
            try {
                var ret = getObject(arg0).history;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_performance_9d639ac65758f7fe = function(arg0) {
        try {
            var ret = getObject(arg0).performance;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_cancelAnimationFrame_4bdbc9fd5f75f9cd = function(arg0, arg1) {
        try {
            try {
                getObject(arg0).cancelAnimationFrame(arg1);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_requestAnimationFrame_c1ca5bf33b036c59 = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
                _assertNum(ret);
                return ret;
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_activeElement_ee2fa296bcface45 = function(arg0) {
        try {
            var ret = getObject(arg0).activeElement;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_createElement_d1b8191d1ca1103b = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_createElementNS_9802e23922dd912b = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            try {
                var ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_createTextNode_0f0e50dff3678aba = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_getElementById_87fd6611f51eaa51 = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_querySelector_10c106d76a42ab14 = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlLiElement_759ce6e5872f3b57 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLLIElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_1a378e8363d89dd8 = function(arg0, arg1) {
        try {
            getObject(arg0).value = arg1;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlTextAreaElement_ac8342fd3f12e7df = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLTextAreaElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_55805ca75f679bbd = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlButtonElement_fd2d14dfe7614228 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLButtonElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_7ce5bf3f70091920 = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_pathname_c20bdf43a9367ec1 = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).pathname;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_search_f2374055000acfbd = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).search;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_hash_3fcc23b12d8eb63c = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).hash;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_new_f021f9f983d35fdc = function(arg0, arg1) {
        try {
            try {
                var ret = new URL(getStringFromWasm0(arg0, arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_newwithbase_cb98c07b89ccc380 = function(arg0, arg1, arg2, arg3) {
        try {
            try {
                var ret = new URL(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlOptionElement_aad8e6885bd94190 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLOptionElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_062dcb9d0db51659 = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_length_9505c5a54e7db43a = function(arg0) {
        try {
            var ret = getObject(arg0).length;
            _assertNum(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_get_f517a2958eaba908 = function(arg0, arg1) {
        try {
            var ret = getObject(arg0)[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_now_0a117b544a88778f = function(arg0) {
        try {
            var ret = getObject(arg0).now();
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlInputElement_4d332a28ab7863fb = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLInputElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_checked_1935800edc06909c = function(arg0, arg1) {
        try {
            getObject(arg0).checked = arg1 !== 0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_type_29981771920678d7 = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).type;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_663d02d42e956b7b = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_selectionStart_b07cd7a8b9c2d0a5 = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg1).selectionStart;
                if (!isLikeNone(ret)) {
                    _assertNum(ret);
                }
                getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
                getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_selectionStart_4c4acc9f5ebb6eb9 = function(arg0, arg1, arg2) {
        try {
            try {
                getObject(arg0).selectionStart = arg1 === 0 ? undefined : arg2 >>> 0;
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_selectionEnd_b205f0210255d548 = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg1).selectionEnd;
                if (!isLikeNone(ret)) {
                    _assertNum(ret);
                }
                getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
                getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_selectionEnd_a0da6031a1626bad = function(arg0, arg1, arg2) {
        try {
            try {
                getObject(arg0).selectionEnd = arg1 === 0 ? undefined : arg2 >>> 0;
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_target_07126e2718b21906 = function(arg0) {
        try {
            var ret = getObject(arg0).target;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_preventDefault_25215e10948cbd7e = function(arg0) {
        try {
            getObject(arg0).preventDefault();
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_pushState_dfd7b08cb8b2ee99 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        try {
            try {
                getObject(arg0).pushState(getObject(arg1), getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlDataElement_fc30cdbc02998bd0 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLDataElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_c363580bd6070a8b = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlParamElement_7eacecb830c2ad8b = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLParamElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_269d1be3d0217637 = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlSelectElement_aa327558667e89e6 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLSelectElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_bdb556d5ba6100ef = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_PopStateEvent_a303b4828c38a5fa = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof PopStateEvent;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_state_d3797c9450dfb4f5 = function(arg0) {
        try {
            var ret = getObject(arg0).state;
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_Element_0383079aab9da573 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof Element;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_namespaceURI_a1c74e4138f60db3 = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).namespaceURI;
            var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_tagName_40e7ed251f26e3d2 = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).tagName;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_closest_3b1096eff7fa79be = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = getObject(arg0).closest(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_getAttribute_026774e1a8620fc8 = function(arg0, arg1, arg2, arg3) {
        try {
            var ret = getObject(arg1).getAttribute(getStringFromWasm0(arg2, arg3));
            var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_getAttributeNames_0031f3760fdf3b43 = function(arg0) {
        try {
            var ret = getObject(arg0).getAttributeNames();
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_removeAttribute_a9581c77eacdef57 = function(arg0, arg1, arg2) {
        try {
            try {
                getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_setAttribute_8fa869e4a7209183 = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            try {
                getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_error_7aac59d937b76b67 = function(arg0) {
        try {
            console.error(getObject(arg0));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlElement_657002a9abe51636 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_focus_be28ba79ae2d3dcf = function(arg0) {
        try {
            try {
                getObject(arg0).focus();
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_addEventListener_540ca8a90a4cfd87 = function(arg0, arg1, arg2, arg3) {
        try {
            try {
                getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_removeEventListener_9f82bb25770ce9c2 = function(arg0, arg1, arg2, arg3) {
        try {
            try {
                getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_href_de40c626fe50b0e4 = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg1).href;
                var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len0 = WASM_VECTOR_LEN;
                getInt32Memory0()[arg0 / 4 + 1] = len0;
                getInt32Memory0()[arg0 / 4 + 0] = ptr0;
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_Node_f3c7285492b0bf19 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof Node;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_nodeType_cc564123cd17fbde = function(arg0) {
        try {
            var ret = getObject(arg0).nodeType;
            _assertNum(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_childNodes_d0b95a2bbfb12b0b = function(arg0) {
        try {
            var ret = getObject(arg0).childNodes;
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_firstChild_a9e5ec8a0416870c = function(arg0) {
        try {
            var ret = getObject(arg0).firstChild;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_nextSibling_36f718775c04bd5a = function(arg0) {
        try {
            var ret = getObject(arg0).nextSibling;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_textContent_177e1cf32d4cc01f = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).textContent;
            var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_textContent_b9cca0a9077046e3 = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_appendChild_9ff018e3b91d6e6b = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg0).appendChild(getObject(arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_insertBefore_cfde74421840f007 = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_removeChild_d6a17858e72dadca = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg0).removeChild(getObject(arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_replaceChild_4c7f2314ebed7577 = function(arg0, arg1, arg2) {
        try {
            try {
                var ret = getObject(arg0).replaceChild(getObject(arg1), getObject(arg2));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlProgressElement_e3dad95113a888db = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLProgressElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_ab70864bb1fc2f12 = function(arg0, arg1) {
        try {
            getObject(arg0).value = arg1;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HashChangeEvent_f45669e1f6919412 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HashChangeEvent;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_newURL_d14a89f01e9e3f0a = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).newURL;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlMenuItemElement_63839f8c852f929f = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLMenuItemElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_checked_35ebae449c130a9c = function(arg0, arg1) {
        try {
            getObject(arg0).checked = arg1 !== 0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlMeterElement_7fd070e6cc5729be = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLMeterElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_bfa3863a2cf634d4 = function(arg0, arg1) {
        try {
            getObject(arg0).value = arg1;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_instanceof_HtmlOutputElement_5668857f8d5727d7 = function(arg0) {
        try {
            var ret = getObject(arg0) instanceof HTMLOutputElement;
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_value_4d99a54af7bfb181 = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).value = getStringFromWasm0(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_forEach_ee047569fb156531 = function(arg0, arg1, arg2) {
        try {
            try {
                var state0 = {a: arg1, b: arg2};
                var cb0 = (arg0, arg1, arg2) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return __wbg_adapter_200(a, state0.b, arg0, arg1, arg2);
                    } finally {
                        state0.a = a;
                    }
                };
                getObject(arg0).forEach(cb0);
            } finally {
                state0.a = state0.b = 0;
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_newnoargs_4f6527054d7f1f1d = function(arg0, arg1) {
        try {
            var ret = new Function(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_call_183c0b733b35a027 = function(arg0, arg1) {
        try {
            try {
                var ret = getObject(arg0).call(getObject(arg1));
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_is_6f42270778843b6d = function(arg0, arg1) {
        try {
            var ret = Object.is(getObject(arg0), getObject(arg1));
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_resolve_a77ae6f272249390 = function(arg0) {
        try {
            var ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_then_695aa7e1c262b929 = function(arg0, arg1) {
        try {
            var ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_then_bca69bfa503c3179 = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_globalThis_eb9027a878db64ad = function() {
        try {
            try {
                var ret = globalThis.globalThis;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_self_69a78003cf074413 = function() {
        try {
            try {
                var ret = self.self;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_window_db757fdea9443777 = function() {
        try {
            try {
                var ret = window.window;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_global_8efdae4f126ac8b4 = function() {
        try {
            try {
                var ret = global.global;
                return addHeapObject(ret);
            } catch (e) {
                handleError(e)
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_closure_wrapper1032 = function(arg0, arg1, arg2) {
        try {
            var ret = makeMutClosure(arg0, arg1, 71, __wbg_adapter_23);
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbindgen_closure_wrapper1030 = function(arg0, arg1, arg2) {
        try {
            var ret = makeMutClosure(arg0, arg1, 69, __wbg_adapter_26);
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbindgen_closure_wrapper6488 = function(arg0, arg1, arg2) {
        try {
            var ret = makeMutClosure(arg0, arg1, 124, __wbg_adapter_20);
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

