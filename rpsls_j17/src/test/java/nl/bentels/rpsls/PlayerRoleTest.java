package nl.bentels.rpsls;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

public class PlayerRoleTest {

    @ParameterizedTest(name="When player 0 overmatches player 1 then player 0 wins")
    @MethodSource("overmatchingPairs")
    public void whenPlayer0OvermatchesPlayer1_thenPlayer0Wins(PlayerRole zero, PlayerRole one) {
        assertEquals(MatchResult.WIN, zero.match(one), () -> "%s should have won from %s".formatted(zero, one));
    }

    private static Stream<Arguments>overmatchingPairs() {
        return Stream.of(
                Arguments.of(new Scissors(), new Paper()),
                Arguments.of(new Scissors(), new Lizard()),
                Arguments.of(new Paper(), new Rock()),
                Arguments.of(new Paper(), new Spock()),
                Arguments.of(new Rock(), new Lizard()),
                Arguments.of(new Rock(), new Scissors()),
                Arguments.of(new Lizard(), new Spock()),
                Arguments.of(new Lizard(), new Paper()),
                Arguments.of(new Spock(), new Scissors()),
                Arguments.of(new Spock(), new Rock())
        );
    }

    @ParameterizedTest(name="When player 0 undermatches player 1 then player 0 loses")
    @MethodSource("undermatchingPairs")
    public void whenPlayer0UndermatchesPlayer1_thenPlayer0Loses(PlayerRole zero, PlayerRole one) {
        assertEquals(MatchResult.LOSE, zero.match(one), () -> "%s should have lost to %s".formatted(zero, one));
    }

    private static Stream<Arguments>undermatchingPairs() {
        return Stream.of(
                Arguments.of(new Scissors(), new Spock()),
                Arguments.of(new Scissors(), new Rock()),
                Arguments.of(new Paper(), new Scissors()),
                Arguments.of(new Paper(), new Lizard()),
                Arguments.of(new Rock(), new Paper()),
                Arguments.of(new Rock(), new Spock()),
                Arguments.of(new Lizard(), new Scissors()),
                Arguments.of(new Lizard(), new Rock()),
                Arguments.of(new Spock(), new Lizard()),
                Arguments.of(new Spock(), new Paper())
        );
    }

    @ParameterizedTest(name="When player 0 same as player 1 then player 0 draws")
    @MethodSource("drawingPairs")
    public void whenPlayer0SameAsPlayer1_thenPlayer0Draws(PlayerRole zero, PlayerRole one) {
        assertEquals(MatchResult.DRAW, zero.match(one), () -> "%s should have drawn with %s".formatted(zero, one));
    }

    private static Stream<Arguments>drawingPairs() {
        return Stream.of(
                Arguments.of(new Scissors(), new Scissors()),
                Arguments.of(new Paper(), new Paper()),
                Arguments.of(new Rock(), new Rock()),
                Arguments.of(new Lizard(), new Lizard()),
                Arguments.of(new Spock(), new Spock())
        );
    }
}
