/**
 * Streaming Ehlers CyberCycle, callable bar-by-bar from JS.
 */
export class CyberCycle {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CyberCycleFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cybercycle_free(ptr, 0);
    }
    /**
     * @param {number} length
     */
    constructor(length) {
        const ret = wasm.cybercycle_new(length);
        this.__wbg_ptr = ret;
        CyberCycleFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Feed one price; returns `[cycle, trigger]`.
     * @param {number} price
     * @returns {Float64Array}
     */
    next(price) {
        const ret = wasm.cybercycle_next(this.__wbg_ptr, price);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
}
if (Symbol.dispose) CyberCycle.prototype[Symbol.dispose] = CyberCycle.prototype.free;

/**
 * Streaming EMA, callable bar-by-bar from JS.
 */
export class Ema {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        EmaFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ema_free(ptr, 0);
    }
    /**
     * @param {number} period
     */
    constructor(period) {
        const ret = wasm.ema_new(period);
        this.__wbg_ptr = ret;
        EmaFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Feed one value; returns the current EMA.
     * @param {number} value
     * @returns {number}
     */
    next(value) {
        const ret = wasm.ema_next(this.__wbg_ptr, value);
        return ret;
    }
}
if (Symbol.dispose) Ema.prototype[Symbol.dispose] = Ema.prototype.free;

/**
 * Streaming harmonic pattern scanner (AB=CD, 5-0, Gartley family).
 */
export class HarmonicScanner {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HarmonicScannerFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_harmonicscanner_free(ptr, 0);
    }
    /**
     * `swing_strength` pivot confirmation bars on each side.
     * @param {number} swing_strength
     */
    constructor(swing_strength) {
        const ret = wasm.harmonicscanner_new(swing_strength);
        this.__wbg_ptr = ret;
        HarmonicScannerFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Feed one `(high, low)` bar; returns newly completed patterns flattened,
     * [`HARMONIC_STRIDE`] values each: `kind, is_bull, x_bar, x_price, a_bar,
     * a_price, b_bar, b_price, c_bar, c_price, d_bar, d_price, score, prz_low,
     * prz_high` (`x_*` is NaN when the pattern has no X point).
     * @param {number} high
     * @param {number} low
     * @returns {Float64Array}
     */
    next(high, low) {
        const ret = wasm.harmonicscanner_next(this.__wbg_ptr, high, low);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * Fields per pattern in the flattened output of [`HarmonicScanner::next`].
     * @returns {number}
     */
    static stride() {
        const ret = wasm.harmonicscanner_stride();
        return ret >>> 0;
    }
}
if (Symbol.dispose) HarmonicScanner.prototype[Symbol.dispose] = HarmonicScanner.prototype.free;

/**
 * Streaming Kagi line builder.
 */
export class Kagi {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KagiFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_kagi_free(ptr, 0);
    }
    /**
     * @param {number} reversal
     */
    constructor(reversal) {
        const ret = wasm.kagi_new(reversal);
        this.__wbg_ptr = ret;
        KagiFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Feed one price; returns `[open, close, direction, thickness]` when the
     * price completes a Kagi line, else an empty array.
     * @param {number} price
     * @returns {Float64Array}
     */
    next(price) {
        const ret = wasm.kagi_next(this.__wbg_ptr, price);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
}
if (Symbol.dispose) Kagi.prototype[Symbol.dispose] = Kagi.prototype.free;

/**
 * Streaming Renko brick builder.
 */
export class Renko {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RenkoFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_renko_free(ptr, 0);
    }
    /**
     * @param {number} box_size
     */
    constructor(box_size) {
        const ret = wasm.renko_new(box_size);
        this.__wbg_ptr = ret;
        RenkoFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Feed one close; returns the bricks it completed, flattened as
     * `[open, close, direction, open, close, direction, ...]` (may be empty).
     * @param {number} close
     * @returns {Float64Array}
     */
    next(close) {
        const ret = wasm.renko_next(this.__wbg_ptr, close);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
}
if (Symbol.dispose) Renko.prototype[Symbol.dispose] = Renko.prototype.free;

/**
 * Streaming RSI, callable bar-by-bar from JS.
 */
export class Rsi {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RsiFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rsi_free(ptr, 0);
    }
    /**
     * `period` Wilder smoothing lookback.
     * @param {number} period
     */
    constructor(period) {
        const ret = wasm.rsi_new(period);
        this.__wbg_ptr = ret;
        RsiFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Feed one `close`; returns the current RSI value.
     * @param {number} close
     * @returns {number}
     */
    next(close) {
        const ret = wasm.rsi_next(this.__wbg_ptr, close);
        return ret;
    }
}
if (Symbol.dispose) Rsi.prototype[Symbol.dispose] = Rsi.prototype.free;

/**
 * Streaming SuperTrend, callable bar-by-bar from JS.
 */
export class SuperTrend {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SuperTrendFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_supertrend_free(ptr, 0);
    }
    /**
     * `period` ATR lookback, `multiplier` band width.
     * @param {number} period
     * @param {number} multiplier
     */
    constructor(period, multiplier) {
        const ret = wasm.supertrend_new(period, multiplier);
        this.__wbg_ptr = ret;
        SuperTrendFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Feed one `(high, low, close)` bar; returns `[supertrend_value, direction]`
     * where `direction` is +1 (uptrend) or -1 (downtrend).
     * @param {number} high
     * @param {number} low
     * @param {number} close
     * @returns {Float64Array}
     */
    next(high, low, close) {
        const ret = wasm.supertrend_next(this.__wbg_ptr, high, low, close);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
}
if (Symbol.dispose) SuperTrend.prototype[Symbol.dispose] = SuperTrend.prototype.free;

/**
 * Name of a harmonic pattern kind code returned by [`HarmonicScanner::next`]
 * (the `HarmonicKind::as_str` value). Any code that is not a known kind index,
 * including NaN, negative or fractional values, returns `"unknown"`.
 * @param {number} code
 * @returns {string}
 */
export function harmonicKindName(code) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.harmonicKindName(code);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_9c31b086c2b26051: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./quantwave_wasm_bg.js": import0,
    };
}

const CyberCycleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cybercycle_free(ptr, 1));
const EmaFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ema_free(ptr, 1));
const HarmonicScannerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_harmonicscanner_free(ptr, 1));
const KagiFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_kagi_free(ptr, 1));
const RenkoFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_renko_free(ptr, 1));
const RsiFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rsi_free(ptr, 1));
const SuperTrendFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_supertrend_free(ptr, 1));

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedFloat64ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
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

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('quantwave_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
