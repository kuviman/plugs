use plugs::Plug;

#[derive(plugs::Bundle)]
struct DepsOfA {}

struct A {
    deps: DepsOfA,
}

#[derive(plugs::Bundle)]
struct DepsOfB {
    a: Plug<A>,
}

struct B {
    deps: DepsOfB,
}

#[derive(plugs::Bundle)]
struct DepsOfC {
    a: Plug<A>,
    b: Plug<B>,
}

struct C {
    deps: DepsOfC,
}

#[derive(plugs::Bundle)]
struct DepsOfD {
    b: Plug<B>,
    c: Plug<C>,
}

struct D {
    deps: DepsOfD,
}

fn main() {
    use plugs::Bundle as _;
    let bundle = plugs::EmptyBundle;
    let a = A {
        deps: bundle.query::<DepsOfA, _>(),
    };
    let bundle = bundle.insert(a);
    let b = B {
        deps: bundle.query::<DepsOfB, _>(),
    };
    let bundle = bundle.insert(b);
    let c = C {
        deps: bundle.query::<DepsOfC, _>(),
    };
    let bundle = bundle.insert(c);
    let d = D {
        deps: bundle.query::<DepsOfD, _>(),
    };
    let bundle = bundle.insert(d);
    #[derive(plugs::Bundle)]
    struct Ctx {
        a: Plug<A>,
        b: Plug<B>,
        c: Plug<C>,
        d: Plug<D>,
    }
    let ctx = bundle.query::<Ctx, _>();
}
