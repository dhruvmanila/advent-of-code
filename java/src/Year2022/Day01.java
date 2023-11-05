package Year2022;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.Reader;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class Day01 {
    private static String inputPath = "./src/Year2022/input/01.txt";

    public static void Solve() throws Exception {
        Reader input = new FileReader(inputPath);
        BufferedReader buffer = new BufferedReader(input);

        List<Integer> elves = new ArrayList<>();
        int elfCalories = 0;

        String line;
        while ((line = buffer.readLine()) != null) {
            if (line.isEmpty()) {
                elves.add(elfCalories);
                elfCalories = 0;
                continue;
            }
            elfCalories += Integer.parseInt(line);
        }
        input.close();

        elves.sort(Comparator.reverseOrder());

        int topCalories = 0;
        for (int calories : elves.subList(0, 3)) {
            topCalories += calories;
        }

        System.out.printf("1.1: %d\n1.2: %d\n", elves.get(0), topCalories);
    }
}
