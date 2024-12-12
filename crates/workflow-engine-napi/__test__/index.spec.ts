import test from 'ava'

import {plus100, asyncPlus100} from '../index.js'

test('sync function from native code', (t) => {
    const fixture = 42
    t.is(plus100(fixture), fixture + 100)
})

test('async function from native code', async (t) => {
    const uint64Max = BigInt("0xFFFFFFFFFFFFFFFF")

    // success
    const successValues = [0n, 42n, uint64Max - 100n]
    for (const v of successValues) {
        t.is(await asyncPlus100(v), v + 100n)
    }

    // fail
    const invalidValues = [-42n, -1n, uint64Max + 1n]
    for (const v of invalidValues) {
        await t.throwsAsync(asyncPlus100(v), {message: /cannot convert/})
    }

    // overflow
    const overflowValues = [uint64Max-99n, uint64Max-1n, uint64Max]
    for (const v of overflowValues) {
        t.is(await asyncPlus100(v), v + 100n - (uint64Max+1n))
    }
})

