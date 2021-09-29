package nl.bentels.rpsls;

import PlayerRole
import kotlin.reflect.KProperty

class Game () {
    var _myRole: PlayerRole? = null
    val myRole: PlayerRole by SelectingDelegate()

    class SelectingDelegate {
        var role: PlayerRole? = null

        operator fun getValue(thisRef: Game?, property: KProperty<*>): PlayerRole {
            if (role == null) {
                role = thisRef?._myRole ?: PlayerRole.random()
            }
            return role as PlayerRole
        }
    }

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
                val myRoleLocal = myRole
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
