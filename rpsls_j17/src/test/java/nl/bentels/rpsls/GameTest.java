package nl.bentels.rpsls;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class GameTest {

    private static final ByteArrayOutputStream BAOS = new ByteArrayOutputStream();

    @BeforeAll
    public static void swapIOStreams() {
        System.setOut(new PrintStream(BAOS));
        System.setIn(new ByteArrayInputStream("spock".getBytes(StandardCharsets.UTF_8)));
    }

    private Game objectUnderTest;

    @BeforeEach
    public void createTestObject() {
        objectUnderTest = new Game(new Spock());
    }

    @Test
    public void whenGameRun_thenInteractionOkay() {
        objectUnderTest.doGame();

        String[] strings = BAOS.toString().split("\n");
        assertEquals(3, strings.length);
        assertEquals("Which role do you want (ROCK, PAPER, SCISSORS, LIZARD, SPOCK)? You chose Spock", strings[0]);
        assertEquals("I chose Spock", strings[1]);
        assertEquals("It's a draw", strings[2]);

    }

}
