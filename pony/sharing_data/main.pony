actor Main
  new create(env: Env) =>
    let a1: A iso = recover iso A end
    let sub = Sub(consume a1)
    // a1.b = B("def") // #2

    let a2: A iso = recover iso A end
    let b: B = B("ghi")
    // a2.b = b // #3

    env.out.print("It works correctly!")

actor Sub
  var _a: A val

  // new create(a: A) => // #1
  new create(a: A iso) =>
    _a = consume a

  fun ref update(a: A val) =>
    _a = a

class A
  var b: B ref

  new create() =>
    b = B("abc")

class B
  var c: String

  new create(s: String) =>
    c = s
