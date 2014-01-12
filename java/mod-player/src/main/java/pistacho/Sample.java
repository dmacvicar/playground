/*
 * (c) 2013 Duncan Mac-Vicar P.
 *
 * Released under the MIT License:
 * http://www.opensource.org/licenses/mit-license.php
 */
package pistacho;

public class Sample {

    private String name = "";
    private long length = 0;
    private int fineTune = 0;
    private int volume = 0;
    private long loopStart = 0;
    private long loopEnd = 0;

    public byte[] getData() {
        return data;
    }

    public void setData(byte[] data) {
        this.data = data;
    }
    byte[] data;

    /**
     * @return the name
     */
    public String getName() {
        return name;
    }

    /**
     * @param name the name to set
     */
    public void setName(String name) {
        this.name = name;
    }

    /**
     * @return the length
     */
    public long getLength() {
        return length;
    }

    /**
     * @param length the length to set
     */
    public void setLength(long length) {
        this.length = length;
    }

    /**
     * @return the fineTune
     */
    public int getFineTune() {
        return fineTune;
    }

    /**
     * @param fineTune the fineTune to set
     */
    public void setFineTune(int fineTune) {
        this.fineTune = fineTune;
    }

    /**
     * @return the volume
     */
    public int getVolume() {
        return volume;
    }

    /**
     * @param volume the volume to set
     */
    public void setVolume(int volume) {
        this.volume = volume;
    }

    /**
     * @return the loopStart
     */
    public long getLoopStart() {
        return loopStart;
    }

    /**
     * @param loopStart the loopStart to set
     */
    public void setLoopStart(long loopStart) {
        this.loopStart = loopStart;
    }

    /**
     * @return the loopEnd
     */
    public long getLoopEnd() {
        return loopEnd;
    }

    /**
     * @param loopEnd the loopEnd to set
     */
    public void setLoopEnd(long loopEnd) {
        this.loopEnd = loopEnd;
    }

}
