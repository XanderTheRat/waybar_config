use std::process::Command;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let etat = ["", "", "", "", ""];

    fn affiche_batterie() -> Result<(usize, String, &'static str), Box<dyn std::error::Error>>{
		let couleur_batterie_bon = "#68d391";
		let couleur_batterie_chargement = "#48bb78";
		let couleur_batterie_branche = "#38b2ac";
		let couleur_batterie_faible = "#f6ad55";
								
		let output_batterie = Command::new("cat").arg("/sys/class/power_supply/BAT0/capacity").output()?;
		let output_batterie_etat = Command::new("cat").arg("/sys/class/power_supply/BAT0/status").output()?;
		let batterie_etat_pas_trim = String::from_utf8_lossy(&output_batterie_etat.stdout);
									
		let batterie_output = String::from_utf8_lossy(&output_batterie.stdout);
		let batterie = batterie_output.trim();
		let batterie_etat = batterie_etat_pas_trim.trim().to_string();
		let batterie_pourcent = batterie.parse::<usize>()?;
		let _: Result<f64, Box<dyn std::error::Error>> = Ok(batterie_pourcent as f64);
		let couleur;

		if batterie_etat == "Charging" {
			couleur = couleur_batterie_chargement;		
		}
		else if batterie_etat == "Not charging" {
			couleur = couleur_batterie_branche;
		}
		else if batterie_pourcent > 30 {
				couleur = couleur_batterie_bon;
		}
		else if batterie_pourcent > 15 {
			couleur = couleur_batterie_faible;
		}
		else {
			couleur = "#FFFFFF"
		}
		Ok((batterie_pourcent, batterie_etat, couleur))							
	}

	fn affichage_tps_restant() -> Result<(i32, i32), Box<dyn std::error::Error>>{
		let energy_now = Command::new("cat").arg("/sys/class/power_supply/BAT0/charge_now").output()?;
		let energy_full = Command::new("cat").arg("/sys/class/power_supply/BAT0/charge_full").output()?;
		let power_now = Command::new("cat").arg("/sys/class/power_supply/BAT0/current_now").output()?;    
		let output_batterie_etat = Command::new("cat").arg("/sys/class/power_supply/BAT0/status").output()?;
			     		
		let energie_restante_pas_trim = String::from_utf8_lossy(&energy_now.stdout);
		let capacite_total_batterie_pas_trim = String::from_utf8_lossy(&energy_full.stdout);
		let energie_consommee_pas_trim = String::from_utf8_lossy(&power_now.stdout);
		let batterie_etat_pas_trim = String::from_utf8_lossy(&output_batterie_etat.stdout); 				
				
		let energie_consommee = energie_consommee_pas_trim.trim().parse::<f64>()?;
		let capacite_total_batterie = capacite_total_batterie_pas_trim.trim().parse::<f64>()?;
		let energie_restante = energie_restante_pas_trim.trim().parse::<f64>()?;
		let batterie_etat = batterie_etat_pas_trim.trim();

		let _ = Ok::<f64, Box<dyn std::error::Error>>(energie_consommee);
		let _ = Ok::<f64, Box<dyn std::error::Error>>(capacite_total_batterie);
		let _ = Ok::<f64, Box<dyn std::error::Error>>(energie_restante);

	    let heures_restantes:f64;
	    
	    if energie_consommee == 0.0 {
        	heures_restantes = 0.0;
    	}
		else if batterie_etat == "Charging" || batterie_etat == "Not Charging" {
        	heures_restantes=(capacite_total_batterie - energie_restante) / energie_consommee;
    	} else {
        	heures_restantes= energie_restante / energie_consommee;
    	}

		let minutes_depuis_heures = heures_restantes * 60.0;
		let mut minutes_restantes = minutes_depuis_heures.round() as i32;
		minutes_restantes /= 60;
		
		Ok((heures_restantes.round() as i32, minutes_restantes))   		    		
	}

	let state_file = "/home/martin/.config/scripts.rs/waybar_scripts/battery_state";
	let mode;

	if Path::new(state_file).exists() {
	        mode = fs::read_to_string(state_file)?.trim().parse::<u8>()?;
			let _: Result<f64, Box<dyn std::error::Error>> = Ok(mode.into());
			match mode {
				1 => {
				let Ok((batterie_pourcent, batterie_etat, couleur)) = affiche_batterie() else { todo!() };
				if batterie_etat == "Charging" {
					println!("<span foreground='{}'>{}% </span>",couleur, batterie_pourcent);		
				}
				else if batterie_etat == "Not charging" {
					println!("<span foreground='{}'>{}% </span>",  couleur, batterie_pourcent)
				}
				else if batterie_pourcent > 30 {
					println!("<span foreground='{}'>{}% {}</span>", couleur, batterie_pourcent, etat[batterie_pourcent/20]);
				}
				else {
						println!("<span foreground={}>{}% {}</span>", couleur, batterie_pourcent, etat[batterie_pourcent/20]);
					}	
				}
				2 => {
					let batterie_etat = Command::new("cat").arg("/sys/class/power_supply/BAT0/status").output()?;
					let batterie_etat_pas_trim = String::from_utf8_lossy(&batterie_etat.stdout);
					let status = batterie_etat_pas_trim.trim().to_string();

					let couleur:&str;
					if status == "Charging" || status == "Not charging" {
						couleur = "#40B792";
						println!("<span foreground='{}'>Batterie en chargement</span>", couleur);
					}else {
						couleur = "#68d391";
						let Ok((heures_restantes, minutes_restantes)) = affichage_tps_restant() else { todo!() };
						println!("<span foreground='{}'>{} heure(s) {} minutes</span>", couleur,heures_restantes, minutes_restantes)
					}
				} 
				_ => {}
		}
	} else {
		println!("Erreur de nom de fichier");
	};

	Ok(())
}				
