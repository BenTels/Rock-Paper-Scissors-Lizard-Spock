import org.junit.jupiter.api.assertAll
import org.junit.jupiter.api.assertThrows
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.MethodSource
import java.util.stream.IntStream
import java.util.stream.Stream
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class PlayerRoleSelectTest {
    @ParameterizedTest(name = "When PlayerRole selected by name, then correct")
    @MethodSource("indexParameters")
    fun whenPlayerRoleSelectedByNameThenCorrect(i: Int, expected: PlayerRole) {
        assertEquals(expected, PlayerRole.byIndex(i), "With index ${i}, should have found ${expected}")
    }

    @Test()
    fun whenNonExistentIndexUsedThenException(){
        assertThrows<NoSuchElementException>("Should have thrown NoSuchElementException") {PlayerRole.byIndex(256)}
    }

    @Test()
    fun whenPlayerRoleRandomlySelectedThenSelectionsEvenlyDistributed() {
        val N = 20000
        val margin = 150
        val numbers = generateSequence(0) {it + 1}
        val counts = numbers.take(N)
            .map { PlayerRole.random() }
            .groupingBy { it }
            .eachCount()
        assertEquals(N, counts.toList().map { it.second }.sum())
        assertAll(
            "Distribution test",
            { assertTrue ( counts[PlayerRole.SCISSORS] in (N/5-margin)..(N/5+margin), "SCISSORS = ${counts[PlayerRole.SCISSORS]} out of expected range" )},
            { assertTrue ( counts[PlayerRole.PAPER] in (N/5-margin)..(N/5+margin), "PAPER = ${counts[PlayerRole.PAPER]} out of expected range" )},
            { assertTrue ( counts[PlayerRole.ROCK] in (N/5-margin)..(N/5+margin), "ROCK = ${counts[PlayerRole.ROCK]} out of expected range" )},
            { assertTrue ( counts[PlayerRole.LIZARD] in (N/5-margin)..(N/5+margin), "LIZARD = ${counts[PlayerRole.LIZARD]} out of expected range" )},
            { assertTrue ( counts[PlayerRole.SPOCK] in (N/5-margin)..(N/5+margin), "SPOCK = ${counts[PlayerRole.SPOCK]} out of expected range" )}
        )
    }

    companion object ParameterGenerator {
        @JvmStatic
        fun indexParameters() = Stream.of(
            Arguments.of(0, PlayerRole.SCISSORS),
            Arguments.of(1, PlayerRole.PAPER),
            Arguments.of(2, PlayerRole.ROCK),
            Arguments.of(3, PlayerRole.LIZARD),
            Arguments.of(4, PlayerRole.SPOCK),
        )
    }

}