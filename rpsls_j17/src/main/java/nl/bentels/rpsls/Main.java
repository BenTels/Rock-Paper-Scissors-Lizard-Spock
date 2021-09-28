package nl.bentels.rpsls;

import java.io.IOException;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) throws IOException {
        boolean playOn = true;
        Scanner scanner = new Scanner(System.in);
        do {
            new Game().doGame();
            System.out.print("Do you want to play another game (Y/n)? ");
            String response = scanner.nextLine();
            if ("n".equalsIgnoreCase(response)) {
                playOn = false;
            }
        } while (playOn);
    }
}
