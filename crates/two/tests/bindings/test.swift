import two

public func test() throws {
  assert(
    EpsilonObject.newDefault()
      == EpsilonObject(
        oneObject: OneObject(one: newOne(value: false)),
        two: newTwo(value: false)
      )
  )
}

try! test()
