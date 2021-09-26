fun main(args: Array<String>) {
    var playOn: Boolean = true
    while (playOn) {
        Game().playGame()
        print("Do you want to play again (Y/n)? ")
        val cont = readLine()
        if ("n".equals(cont, ignoreCase = true)) {
            playOn = false
        }
    }
}