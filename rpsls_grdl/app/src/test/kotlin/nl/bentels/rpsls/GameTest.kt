package nl.bentels.rpsls;

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance
import java.io.*

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class GameTest {

    val bos = ByteArrayOutputStream()

    @BeforeEach
    fun initIO (){
        println("Initializing...")
        val spock = PlayerRole.SPOCK.name.byteInputStream()
        System.setIn(spock)
        System.setOut(PrintStream(bos))
    }

    @Test
    fun whenGameExercisedThenOkay() {
        Game(PlayerRole.SPOCK).playGame()
        val resStrings = bos.toString().split('\n')
        print(resStrings)
        assertEquals(4, resStrings.size)
        assertEquals("Which role do you want (SCISSORS, PAPER, ROCK, LIZARD, SPOCK)? You chose SPOCK", resStrings[0])
        assertEquals("I chose SPOCK", resStrings[1])
        assertEquals("It's a DRAW", resStrings[2])
    }

}
