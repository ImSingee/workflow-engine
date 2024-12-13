import { core, internals, primordials } from "ext:core/mod.js";
import * as url from "ext:deno_url/00_url.js";
import * as webidl from "ext:deno_webidl/00_webidl.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import * as event from "ext:deno_web/02_event.js";
import * as timers from "ext:deno_web/02_timers.js";
import * as abortSignal from "ext:deno_web/03_abort_signal.js";
import * as globalInterfaces from "ext:deno_web/04_global_interfaces.js";
import * as base64 from "ext:deno_web/05_base64.js";
import * as streams from "ext:deno_web/06_streams.js";
import * as encoding from "ext:deno_web/08_text_encoding.js";
import * as fileReader from "ext:deno_web/10_filereader.js";
import * as location from "ext:deno_web/12_location.js";
import * as messagePort from "ext:deno_web/13_message_port.js";
import * as console from "ext:deno_console/01_console.js";

import * as unused1 from "ext:deno_web/15_performance.js";
import * as unused2 from "ext:deno_url/01_urlpattern.js";
import * as unused3 from "ext:deno_web/16_image_data.js";
import * as unused4 from "ext:deno_web/14_compression.js";
import * as unused5 from "ext:deno_web/09_file.js";

const {
    ArrayPrototypeFilter,
    ArrayPrototypeForEach,
    ArrayPrototypeIncludes,
    ArrayPrototypeMap,
    Error,
    ErrorPrototype,
    FunctionPrototypeBind,
    FunctionPrototypeCall,
    ObjectAssign,
    ObjectDefineProperties,
    ObjectDefineProperty,
    ObjectHasOwn,
    ObjectKeys,
    ObjectGetOwnPropertyDescriptor,
    ObjectGetOwnPropertyDescriptors,
    ObjectPrototypeIsPrototypeOf,
    ObjectSetPrototypeOf,
    PromisePrototypeThen,
    PromiseResolve,
    StringPrototypePadEnd,
    Symbol,
    SymbolIterator,
    TypeError,
    uncurryThis,
} = primordials;

const windowOrWorkerGlobalScope = {
    setInterval: core.propWritable(timers.setInterval),
    setTimeout: core.propWritable(timers.setTimeout),
}


ObjectDefineProperties(globalThis, windowOrWorkerGlobalScope);


