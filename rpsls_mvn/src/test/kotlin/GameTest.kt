import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.Test
import java.io.*
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class GameTest {

    companion object initializer {

        val bos = ByteArrayOutputStream()

        @JvmStatic
        @BeforeAll
        fun initIO (){
            val spock = PlayerRole.SPOCK.name.byteInputStream()
            System.setIn(spock)
            System.setOut(PrintStream(bos))
        }
    }

    @Test
    fun whenGameExercisedThenOkay() {
        Game(PlayerRole.SPOCK).playGame()
        val resStrings = GameTest.bos.toString().split('\n')
        print(resStrings)
        assertEquals(4, resStrings.size)
        assertEquals("Which role do you want (SCISSORS, PAPER, ROCK, LIZARD, SPOCK)? You chose SPOCK", resStrings[0])
        assertEquals("I chose SPOCK", resStrings[1])
        assertEquals("It's a DRAW", resStrings[2])
    }
}