import kotlin.math.abs
import kotlin.random.Random

enum class PlayerRole(val rank: Int) {
    SCISSORS(0),
    PAPER(1),
    ROCK(2),
    LIZARD(3),
    SPOCK(4);

    fun compareWith(other: PlayerRole): RoleComparisonResult {
        var compareWith: RoleComparisonResult = RoleComparisonResult.DRAW
        if (rank != other.rank) {
            val rankDiff = when (0 < other.rank - rank) {
                true -> other.rank - rank
                false -> abs(other.rank - rank -1)
            }
            compareWith = when (rankDiff % 2) {
                0 -> RoleComparisonResult.LOSE
                1 -> RoleComparisonResult.WIN
                else -> throw IllegalStateException("Kotlin knows nothing of math")
            }
        }
        return compareWith
    }

    companion object Factory {
        private val _random: Random = Random.Default

        internal fun byIndex(i : Int) : PlayerRole =
            when (i) {
                0 -> SCISSORS
                1 -> PAPER
                2 -> ROCK
                3 -> LIZARD
                4 -> SPOCK
                else -> throw NoSuchElementException()
            }

        fun random() = byIndex(_random.nextInt(until = 5))
    }
}