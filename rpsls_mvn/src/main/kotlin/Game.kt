class Game () {
    internal var _myRole: PlayerRole? = null
    val myRole
        get() = _myRole ?: PlayerRole.random()

    constructor(forcedRole: PlayerRole) : this() {
        _myRole = forcedRole
    }

    fun playGame(): Unit {
        var chosen = false
        while (!chosen) {
            try {
                print("Which role do you want (${PlayerRole.values().joinToString()})? ")
                val choice = readLine()!!
                val chosenRole = PlayerRole.valueOf(choice.uppercase())
                chosen = true
                val result = chosenRole.compareWith(myRole)
                println("You chose ${choice.uppercase()}")
                println("I chose ${myRole}")
                println(
                    when (result) {
                        RoleComparisonResult.WIN, RoleComparisonResult.LOSE -> "You ${result}"
                        RoleComparisonResult.DRAW -> "It's a ${result}"
                    }
                )
            } catch (nsee: NoSuchElementException) {
                println("That's not a role!! Try again...")
            }
        }
    }
}