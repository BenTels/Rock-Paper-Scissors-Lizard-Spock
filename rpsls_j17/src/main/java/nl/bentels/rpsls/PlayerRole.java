package nl.bentels.rpsls;

public sealed interface PlayerRole permits
    Scissors, Paper, Rock, Lizard, Spock {

    int getOrder();

    default MatchResult match(PlayerRole other) {
        final int diff =
                0 <= getOrder() - other.getOrder() ?
                        getOrder() - other.getOrder() :
                        Math.abs(getOrder() - other.getOrder() -1);
        return switch (diff) {
          case 0 -> MatchResult.DRAW;
            default -> switch (diff % 2) {
                case 0 -> MatchResult.WIN;
                case 1 -> MatchResult.LOSE;
                default -> throw new IllegalStateException("Java cannot do maths");
            };
        };
    }
}

final class Scissors implements PlayerRole {

    @Override
    public int getOrder() {
        return 0;
    }
}

final class Paper implements PlayerRole {

    @Override
    public int getOrder() {
        return 1;
    }
}

final class Rock implements PlayerRole {

    @Override
    public int getOrder() {
        return 2;
    }
}

final class Lizard implements PlayerRole {

    @Override
    public int getOrder() {
        return 3;
    }
}

final class Spock implements PlayerRole {

    @Override
    public int getOrder() {
        return 4;
    }
}


