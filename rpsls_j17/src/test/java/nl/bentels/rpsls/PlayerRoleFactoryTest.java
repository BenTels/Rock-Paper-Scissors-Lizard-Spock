package nl.bentels.rpsls;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;
import org.junit.jupiter.params.provider.ValueSource;

import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

public class PlayerRoleFactoryTest {

    @ParameterizedTest(name="When role generated by name then correct class")
    @MethodSource("namesAndClasses")
    public void whenRoleGeneratedByNameThenCorrectClassReturned(String name, Class<PlayerRole> expected) {
        assertEquals(expected,
                PlayerRoleFactory.fromName(name).getClass(),
                () -> "For role %s we should have gotten an instance of %s".formatted(name, expected.getName()));
    }

    private static Stream<Arguments> namesAndClasses() {
        return Stream.of(
           Arguments.of("Scissors", Scissors.class),
           Arguments.of("scissors", Scissors.class),
           Arguments.of("SCISSORS", Scissors.class),
           Arguments.of("Paper", Paper.class),
           Arguments.of("paper", Paper.class),
           Arguments.of("PAPER", Paper.class),
           Arguments.of("Rock", Rock.class),
           Arguments.of("rock", Rock.class),
           Arguments.of("ROCK", Rock.class),
           Arguments.of("Lizard", Lizard.class),
           Arguments.of("lizard", Lizard.class),
           Arguments.of("LIZARD", Lizard.class),
           Arguments.of("Spock", Spock.class),
           Arguments.of("spock", Spock.class),
           Arguments.of("SPOCK", Spock.class)
        );
    }

    @Test
    public void whenRolesSelectedRandomly_thenRolesRandomlyDistributed() {
        final int totalDraws = 100_000;
        final int expectedCount = totalDraws / 5;
        final int margin = totalDraws / 100 * 2;
        Map<Class<? extends PlayerRole>, AtomicInteger> typeCounters = Map.of(
                Scissors.class, new AtomicInteger(0),
                Paper.class, new AtomicInteger(0),
                Rock.class, new AtomicInteger(0),
                Lizard.class, new AtomicInteger(0),
                Spock.class, new AtomicInteger(0)
        );
        int draw = 0;
        while (draw < totalDraws) {
            typeCounters.get(PlayerRoleFactory.random().getClass()).incrementAndGet();
            draw++;
        }
        assertAll(
                () -> assertTrue(expectedCount - margin <= typeCounters.get(Scissors.class).get()
                                    && typeCounters.get(Scissors.class).get() <= expectedCount + margin,
                        () -> "SCISSORS count is out of bounds"),
                () -> assertTrue(expectedCount - margin <= typeCounters.get(Paper.class).get()
                                    && typeCounters.get(Paper.class).get() <= expectedCount + margin,
                        () -> "PAPER count is out of bounds"),
                () -> assertTrue(expectedCount - margin <= typeCounters.get(Rock.class).get()
                                    && typeCounters.get(Rock.class).get() <= expectedCount + margin,
                        () -> "ROCK count is out of bounds"),
                () -> assertTrue(expectedCount - margin <= typeCounters.get(Lizard.class).get()
                                    && typeCounters.get(Lizard.class).get() <= expectedCount + margin,
                        () -> "LIZARD count is out of bounds"),
                () -> assertTrue(expectedCount - margin <= typeCounters.get(Spock.class).get()
                                    && typeCounters.get(Spock.class).get() <= expectedCount + margin,
                        () -> "SPOCK count is out of bounds")
        );
    }
}