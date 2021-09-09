"use strict";

const addon = require("./index.node");
const { makeIterable, iterableNext } = addon;

class Iterator {
    constructor(n) {
        this.iterable = makeIterable(n);
    }

    next() {
        const value = iterableNext(this.iterable);

        if (value === undefined) {
            return { done: true };
        }

        return { value }
    }
}

class Iterable {
    constructor(n) {
        this.n = n;
    }

    [Symbol.iterator]() {
        return new Iterator(this.n);
    }
}

module.exports = {
    ...addon,
    Iterable,
};
