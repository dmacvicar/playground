package pistacho;

import java.io.ByteArrayInputStream;
import javax.sound.sampled.AudioFormat;
import javax.sound.sampled.AudioInputStream;
import javax.sound.sampled.AudioSystem;
import javax.sound.sampled.Clip;
import javax.sound.sampled.DataLine;
import javax.sound.sampled.Line;
import javax.sound.sampled.LineUnavailableException;
import javax.sound.sampled.Mixer;
import javax.sound.sampled.SourceDataLine;
import pistacho.loader.MODLoader;

public class Player {

    private Module module;

    public Player(Module module) {
        this.module = module;
    }

    private void playSamples() throws Exception {
        int numSamples = this.module.getSamples().size();
        Clip[] clips = new Clip[numSamples];
        for (int i=0; i < numSamples; i++) {
            clips[i] = AudioSystem.getClip();
            Sample sample = this.module.getSamples().get(i);
            if (sample.getLength() > 0) {
              System.out.println("Size: " +  this.module.getSamples().get(i).getData().length);
              AudioInputStream audioIn = AudioSystem.getAudioInputStream(
                      new ByteArrayInputStream(
                              this.module.getSamples().get(i).getData())
                      );
              clips[i].open(audioIn);
            }
        }
    }
    
    private void printAudioInfo() throws LineUnavailableException {
        Mixer.Info[] mi = AudioSystem.getMixerInfo();
        for (Mixer.Info info : mi) {
            System.out.println("info: " + info);
            Mixer m = AudioSystem.getMixer(info);
            System.out.println("mixer " + m);
            Line.Info[] sl = m.getSourceLineInfo();
            for (Line.Info info2 : sl) {
                System.out.println("    info: " + info2);
                Line line = AudioSystem.getLine(info2);
                if (line instanceof SourceDataLine) {
                    SourceDataLine source = (SourceDataLine) line;

                    DataLine.Info i = (DataLine.Info) source.getLineInfo();
                    for (AudioFormat format : i.getFormats()) {
                        System.out.println("    format: " + format);
                    }
                }
            }
        }
    }

    public void play() throws Exception {
        //printAudioInfo();
        playSamples();
        for (Pattern pat : module.getOrder()) {
            for (int rowIdx = 0; rowIdx < pat.getNumberOfRows(); rowIdx++) {
                for (int chanIdx = 0; chanIdx < pat.getNumberOfChannels(); chanIdx++) {
                    Pattern.Cell cell = pat.get(rowIdx, chanIdx);
                    int note = cell.periodFrequency;
                    String sampleName = (cell.sample != null ? cell.sample.getName() : "");
                    System.out.print(String.format("| %d %s ", note, sampleName));
                }
                System.out.print("\n");
            }
        }
    }

    public static void main(String[] args) throws Exception {
        Loader loader = new MODLoader();
        Module mod = loader.loadFile("/home/duncan/Downloads/mod.bobofmop.mod");
        Player player = new Player(mod);
        player.play();

    }
}
