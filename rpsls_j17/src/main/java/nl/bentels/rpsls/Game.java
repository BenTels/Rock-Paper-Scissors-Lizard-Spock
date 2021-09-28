package nl.bentels.rpsls;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.StringReader;
import java.util.Scanner;

public class Game {
    private final PlayerRole ourRole;

    public Game() {
        this(PlayerRoleFactory.random());
    }

    Game(PlayerRole ourRole) {
        this.ourRole = ourRole;
    }

    public void doGame() {
        boolean chosen = false;
        do {
            try {
                System.out.print("Which role do you want (ROCK, PAPER, SCISSORS, LIZARD, SPOCK)? ");
                Scanner scanner = new Scanner(System.in);
                PlayerRole chosenRole = PlayerRoleFactory.fromName(scanner.nextLine());
                chosen = true;
                System.out.println("You chose %s".formatted(chosenRole.getClass().getSimpleName()));
                System.out.println("I chose %s".formatted(ourRole.getClass().getSimpleName()));
                System.out.println(
                        switch (chosenRole.match(ourRole)) {
                            case WIN -> "You win!!";
                            case LOSE -> "You lose..";
                            case DRAW -> "It's a draw";
                        }
                );
            } catch (IllegalArgumentException iae) {
                System.out.println(iae.getMessage());
                System.out.println("Try again");
            }
        } while (!chosen);
    }
}
