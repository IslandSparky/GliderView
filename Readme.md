#GliderView
This application is used in conjunction with GliderView to look for Game of Life "gliders" (also called
"spaceships") which are patterns in the Game of Life that move across the "world" in a way that preserves
their form. This application is part of a pair used to search for larger gliders (those above 5 active cells).

This program is used to analyze the "preview.dat" files written by the GliderTrap application.  This
file contains records saving the random starting configurations for runs of the game that produce
outcomes that reach the edge of the game "world". A large percentage of the records in this file (about
9 out of 10) do not represent interesting glider finds.  Instead, they represent cases where the game has
simply grown to reach the game boundaries. True large gliders are evident by the fact that they reach the
world boundary well ahead of the normal spread of the patterns in the world.

By using this program on this preview.dat file, the user quickly review the records in preview.dat to see if they produce big gliders of interest.  With practice, this can be done quickly with the display turned off by just looking at the final result of each record's run at the final state with the display is automatically turned back on.  This all will make more sense with usuage. 

When a true large glider of interest is found, the "s" key can be used to save the intial configuration to the "saved.dat" file for later review and archival.

The program also provides for looking at the saved large gliders in"saved.dat". At the beginning of the program the user is prompted as to whether preview.dat or saved.dat should be opened for the source of the initial configuration records. 

## Usage for preview mode
Assuming the user wants to initially preview the output of GliderTrap, which is the usual case, the "preview.dat" file should be moved from the home directory of GliderTrap into the home directoy of GliderView. A new copy of "preview.dat" will be generated on the next execution of GliderTrap.

When GliderView is executed, it will first ask if the user wants to preview the Gliderview file ("preview.dat") or look at the saved interesting results of previous executions ("saved.dat"). Choose the 'p' option to preview.

The program will then open the world view and begin executing the saved intial state from the first record of the preview file. It will stop when the boundaries of the world are reached. At this time the
user should click in the world display to put focus there. 
 
During execution, assuming focus is on the world display, the following keys are available to control the execution of the Game of Life.

'left arrow' pauses execution.

'right arrow' resumes execution.

'up arrow' speeds up execution. It may be held down to rapidly increase game speed.

'down arrow' slows execution to one move per second in order to examine details. 

'd' key toggles the display mode.  When display is turned off, the game will very rapidly execute until
        the final state when the world boundary is reached. Display will then be automatically restored.

'r' key replays the current world   so that execution may be re-examined. 

'space' move the game to the next record in the input file.

's' key saves the current record to the "saved.dat" file where interesting gliders or other artifacts interest are saved. 

## Usuage for saved mode

The program may also be used to examine the saved contents of "saved.dat".  In this case, the 's' option should be selected from the intial prompt.  In this case the options are the same as above except that the "save" option is not available as we are already reading from the "saved.dat" file. 

