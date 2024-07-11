import com.sajjon.one.*
import com.sajjon.two.*

fun test() {
    assert(EpsilonObject.newDefault() == EpsilonObject(
        oneObject = OneObject(one = newOne(value = false)),
        two = newTwo(value = false)
        )
    )
}

test()