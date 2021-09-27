import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.MethodSource
import java.util.stream.Stream
import kotlin.test.assertEquals

class PlayerRoleWinLoseTest {
    @ParameterizedTest()
    @MethodSource("winCombinations")
    fun whenFirstRoleShouldWin_thenResultIsWin(left : PlayerRole, right: PlayerRole) {
        assertEquals(RoleComparisonResult.WIN, left.compareWith(right), "${left} should have defeated ${right}")
    }

    @ParameterizedTest()
    @MethodSource("loseCombinations")
    fun whenFirstRoleShouldLose_thenResultIsLose(left : PlayerRole, right: PlayerRole) {
        assertEquals(RoleComparisonResult.LOSE, left.compareWith(right), "${left} should have lost to ${right}")
    }

    @ParameterizedTest()
    @MethodSource("drawCombinations")
    fun whenFirstRoleShouldDraw_thenResultIsDraw(left : PlayerRole, right: PlayerRole) {
        assertEquals(RoleComparisonResult.DRAW, left.compareWith(right), "${left} should have drawn to ${right}")
    }

    private companion object {
        @JvmStatic
        fun winCombinations() = Stream.of(
            Arguments.of(PlayerRole.SCISSORS, PlayerRole.PAPER),
            Arguments.of(PlayerRole.SCISSORS, PlayerRole.LIZARD),
            Arguments.of(PlayerRole.PAPER, PlayerRole.ROCK),
            Arguments.of(PlayerRole.PAPER, PlayerRole.SPOCK),
            Arguments.of(PlayerRole.ROCK, PlayerRole.SCISSORS),
            Arguments.of(PlayerRole.ROCK, PlayerRole.LIZARD),
            Arguments.of(PlayerRole.LIZARD, PlayerRole.SPOCK),
            Arguments.of(PlayerRole.LIZARD, PlayerRole.PAPER),
            Arguments.of(PlayerRole.SPOCK, PlayerRole.SCISSORS),
            Arguments.of(PlayerRole.SPOCK, PlayerRole.ROCK)
        )

        @JvmStatic
        fun loseCombinations() = Stream.of(
            Arguments.of(PlayerRole.SCISSORS, PlayerRole.SPOCK),
            Arguments.of(PlayerRole.SCISSORS, PlayerRole.ROCK),
            Arguments.of(PlayerRole.PAPER, PlayerRole.SCISSORS),
            Arguments.of(PlayerRole.PAPER, PlayerRole.LIZARD),
            Arguments.of(PlayerRole.ROCK, PlayerRole.PAPER),
            Arguments.of(PlayerRole.ROCK, PlayerRole.SPOCK),
            Arguments.of(PlayerRole.LIZARD, PlayerRole.ROCK),
            Arguments.of(PlayerRole.LIZARD, PlayerRole.SCISSORS),
            Arguments.of(PlayerRole.SPOCK, PlayerRole.LIZARD),
            Arguments.of(PlayerRole.SPOCK, PlayerRole.PAPER)
        )

        @JvmStatic
        fun drawCombinations() = Stream.of(
            Arguments.of(PlayerRole.SCISSORS, PlayerRole.SCISSORS),
            Arguments.of(PlayerRole.PAPER, PlayerRole.PAPER),
            Arguments.of(PlayerRole.ROCK, PlayerRole.ROCK),
            Arguments.of(PlayerRole.LIZARD, PlayerRole.LIZARD),
            Arguments.of(PlayerRole.SPOCK, PlayerRole.SPOCK),
        )
    }

}