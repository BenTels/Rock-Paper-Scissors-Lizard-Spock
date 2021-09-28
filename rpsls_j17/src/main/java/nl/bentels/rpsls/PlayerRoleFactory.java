package nl.bentels.rpsls;

import java.util.Locale;
import java.util.concurrent.ThreadLocalRandom;

public class PlayerRoleFactory {
    public static PlayerRole fromName(final String name) {
        String upName = name == null ? "" : name.toUpperCase(Locale.ROOT);
        return switch (upName) {
          case "SCISSORS" -> new Scissors();
          case "PAPER" -> new Paper();
          case "ROCK" -> new Rock();
          case "LIZARD" -> new Lizard();
          case "SPOCK" -> new Spock();
          default -> throw new IllegalArgumentException("That's not a player role name!!");
        };
    }

    public static PlayerRole random() {
        PlayerRole[] roles = new PlayerRole[] {new Scissors(), new Paper(), new Rock(), new Lizard(), new Spock()};
        return roles[ThreadLocalRandom.current().nextInt(roles.length)];
    }
}
