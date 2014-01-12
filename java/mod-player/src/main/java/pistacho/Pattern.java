/*
 * (c) 2013 Duncan Mac-Vicar P.
 *
 * Released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package pistacho;

public class Pattern {

    public static class Cell {
        public Sample sample = null;
        public int periodFrequency = 0;
        public int effectNumber = 0;
        public int effectParam = 0;

    }
    int numberOfChannels = 0;

    private final Cell[][] cells;

    public Pattern(int numberOfChannels, int len) {
        cells = new Cell[numberOfChannels][len];
        for (int chan=0; chan < numberOfChannels; chan++) {
            for (int k=0; k < len; k++) {
                cells[chan][k] = new Cell();
            }
        }
    }

    public Cell get(int channel, int row) {
        return cells[channel][row];
    }

    public void set(int channel, int row, Cell cell) {
        cells[channel][row] = cell;
    }

    public void setSample(int channel, int row, Sample sample) {
        cells[channel][row].sample = sample;
    }

    public void setPeriodFrequency(int channel, int row, int freq) {
        cells[channel][row].periodFrequency = freq;
    }
}
