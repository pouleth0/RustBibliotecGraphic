extern crate ldap3;
mod loadMenu;
mod app {
    pub mod menuMain;
}
use ldap3::{LdapConn, LdapConnSettings};
use app::menuMain;

fn main() {
    // Configurações de conexão LDAP
    let ldap_server = "192.168.1.1:389";
    let ldap_dn = "DC=server,DC=com";
    let ldap_user = "username";
    let ldap_password = "password";

    // Conectando ao servidor LDAP
    let ldap_url = format!("ldap://{}", ldap_server);
    let ldap_conn = LdapConn::new(&ldap_url)?;

    // Autenticando com as credenciais fornecidas
    ldap_conn.simple_bind(&ldap_user, &ldap_password)?;

    // Verificando os grupos aos quais o usuário pertence
    let groups = ldap_conn
        .search(&ldap_dn, ldap3::Scope::Subtree, "(samAccountName=USERNAME_HERE)")
        .unwrap();  // Substitua USERNAME_HERE pelo nome de usuário

    // Liberando os botões de acordo com os grupos
    for group in groups {
        let group_dn = group.dn.unwrap_or_default();
        match group_dn {
            "OU=departamentos,OU=usuarios,OU=diretoria,OU=ti,OU=pessoal,OU=fiscal,OU=contabil,OU=paralegal" => {
                // Libere os botões no menu de acordo com a regra especificada
                loadMenu::show_menu_painel();
            }
            _ => {
                // Não faz nada para grupos desconhecidos
            }
        }
    }

    // Carregando o menu de painel
    menuMain::show_menu_window();
    
}

fn menu_painel() {
    // Carregando o menu de painel e configurando a janela
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_position(gtk::WindowPosition::Left);
    window.set_default_size(800, 600);
    window.show_all();
}