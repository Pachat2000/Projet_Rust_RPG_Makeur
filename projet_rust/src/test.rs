use yaml_rust::YamlLoader;
use yaml_rust::YamlEmitter;
fn main(){
    let histoire =
        "
        1:
            -C’est le matin, de la lumière orangée passe à travers vos rideaux de toile fine et brunie par le temps. Ils sont si anciens que vous ne vous souvenez plus de leur couleur originale… gris ? Marron peut-être… De toute façon vous n’en avez que faire, c’est la dernière fois que vous aurez l'occasion de les voir. Aujourd’hui vous partez.
            « Cette vie n’est pas faite pour moi» vous vous dites en regardant la tâche de moisie au-dessus de votre lit qui ressemble vaguement à un visage. Vous préparez rapidement vos affaires et ramassez quelque provision de celle qui ne semble pas encore létale (un morceau de pain sec) et une dague à la lame bleutée .
            Depuis le retour des monstres, vampires et autre hamster tueur, la famine s’est abattue sur le royaume. Vous ne savez pas à quoi vous attendre mais vous n’avez pas le choix, vous allez mourir de faim si vous ne tentez rien.
            
            De plus, le roi rongé par la maladie, sans successeur, met le pays entier dans une position délicate. Il a fait distribuer dans tout le royaume des invitations à une sorte de spectacle lors duquel il choisira la personne qui le remplacera.
            « Qui sait » vous vous êtes dit en la lisant «Je n’ai absolument rien à faire, rien à manger et aucun background ! C’est presque comme si la personne qui m'avait écrit avait eu la flemme d’en créer et qu'elle devait trouver une raison afin de commencer une aventure le plus vite possible sans raison particulière pour ne pas perdre l’intérêt des lecteurs. Je devrais y aller ! ».
            Vous êtes face à votre demeure… Enfin... vôtre cabane, l'aventure commence ! Que faites-vous?
        2:
            - va bruler la maison
        3:
            - va lire la doc 
        ";

    let docs = YamlLoader::load_from_str(histoire).unwrap();
    let doc = &docs[0];

    // Debug support
    println!("{:?}", doc);
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}