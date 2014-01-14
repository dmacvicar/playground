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
        this.numberOfChannels = numberOfChannels;
        this.cells = new Cell[len][numberOfChannels];
        for (int k = 0; k < len; k++) {
            for (int chan = 0; chan < numberOfChannels; chan++) {
                this.cells[k][chan] = new Cell();
            }
        }
    }

    public Cell get(int row, int channel) {
        return this.cells[row][channel];
    }

    public int getNumberOfRows() {
        return this.cells.length;
    }

    public int getNumberOfChannels() {
        return this.numberOfChannels;
    }

    public void set(int row, int channel, Cell cell) {
        this.cells[row][channel] = cell;
    }

    public void setSample(int row, int channel, Sample sample) {
        this.cells[row][channel].sample = sample;
    }

    public void setPeriodFrequency(int row, int channel, int freq) {
        this.cells[row][channel].periodFrequency = freq;
    }
}
