
pub fn instructions(){
    const MESSAGE_TO_YOU: &str = 
    "\nWELCOME TO YOUR VOLUMES CALCULATOR\n
    \n
    Run \'cargo r -- <solid type> -<cubic unit>'\n
    Args to <solid type> are:\n
    <cy> for cyllinder\n
    <sp> for sphere\n
    <co> for cone\n
    <py> for pyramid\n
    <pr> for prism\n
    <cb> for cube\n
    <rec> for rectangle\n
    \n
    Args to <cubic unit> are:\n
    <-cm> for cubic centimeter\n
    <-i> for cubic inch\n
    <-f> for cubic feet\n
    <-m> for cubic meter\n
    <-mm> for cubic millimeter\n
    For example to know the volume\n
    of a cube in feet³, tip ´cargo r -- cb -f´.
    \n
    END.";
    println!("{}", MESSAGE_TO_YOU);
}


