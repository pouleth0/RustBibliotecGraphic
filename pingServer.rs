extern crate reqwest;

fn main() {
    // Configuração do servidor LDAP
    let ldap_server = "192.168.1.1";
    let ldap_port = 389;

    // Verificando a disponibilidade do servidor
    let url = format!("http://{}:{}", ldap_server, ldap_port);
    match reqwest::get(&url) {
        Ok(response) => {
            if response.status().is_success() {
                println!("O servidor está online.");
            } else {
                println!("O servidor está offline.");
            }
        }
        Err(_) => {
            println!("O servidor está offline.");
        }
    }
}