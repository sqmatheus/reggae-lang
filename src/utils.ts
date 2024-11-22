export type Result<T, E> = Ok<T> | Err<E>;

class Ok<T> {
    readonly isOk = true;
    readonly isErr = false;

    constructor(public readonly value: T) { }

    unwrap(): T {
        return this.value;
    }
}

class Err<E> {
    readonly isOk = false;
    readonly isErr = true;

    constructor(public readonly error: E) { }

    unwrap(): never {
        throw new Error(`Tried to unwrap an Err value: ${this.error}`);
    }
}

export function ok<T, E>(value: T): Result<T, E> {
    return new Ok(value);
}

export function err<T, E>(error: E): Result<T, E> {
    return new Err(error);
}

export type Option<T> = Some<T> | None;

class Some<T> {
    readonly isSome = true;
    readonly isNone = false;

    constructor(public readonly value: T) { }

    unwrap(): T {
        return this.value;
    }
}

class None {
    readonly isSome = false;
    readonly isNone = true;

    unwrap(): never {
        throw new Error("Tried to unwrap a None value");
    }
}

export function some<T>(value: T): Option<T> {
    return new Some(value);
}

export function none<T>(): Option<T> {
    return new None();
}