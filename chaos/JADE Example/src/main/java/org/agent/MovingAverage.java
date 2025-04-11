package org.agent;

import java.util.ArrayList;
import java.util.List;

public class MovingAverage {
    private List<Double> values;
    private int windowSize;
    private double sum;

    public MovingAverage(int windowSize) {
        this.values = new ArrayList<>();
        this.windowSize = windowSize;
        this.sum = 0.0;
    }

    public void addValue(double value) {
        values.add(value);
        sum += value;

        // Überprüfe, ob die Liste größer ist als das Fenster, und entferne das älteste Element
        if (values.size() > windowSize) {
            double removedValue = values.remove(0);
            sum -= removedValue;
        }
    }

    public double getMovingAverage() {
        if (values.isEmpty()) {
            return 0.0; // Oder eine andere Standard-Rückgabewert, wenn die Liste leer ist.
        }
        return sum / values.size();
    }
}
