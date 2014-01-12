/*
 * (c) 2013 Duncan Mac-Vicar P.
 *
 * Released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package pistacho;

import java.util.ArrayList;
import java.util.List;

public class Module {

    private String name = "";
    private int numberOfChannels = 0;
    private final List<Sample> samples = new ArrayList<Sample>();
    private final List<Pattern> order = new ArrayList<Pattern>();
    private final List<Pattern> patterns = new ArrayList<Pattern>();

    public List<Pattern> getPatterns() {
        return patterns;
    }

    public Module() {
    }

    public int getSongLength() {
        return order.size();
    }

    public List<Pattern> getOrder() {
        return order;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public int getNumberOfChannels() {
        return numberOfChannels;
    }

    public void setNumberOfChannels(int numberOfChannels) {
        this.numberOfChannels = numberOfChannels;
    }

    public String toString() {
        return String.format("%s - %d", getName(), getNumberOfChannels());
    }

    public List<Sample> getSamples() {
        return samples;
    }
}
