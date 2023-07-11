#!/bin/bash

release() {
    project_version=$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name == "arcanist").version')
    echo "Project version ('Cargo.toml') : ${project_version}"

    install_version=$(sed -n "s/VERSION=\"\([^\"]*\)\"/\1/p" install.sh)
    echo -n "Install version ('install.sh') : ${install_version}"
    if [[ "$project_version" != "$install_version" ]]; then
        echo " -> ${project_version}"
        sed -i "s/VERSION=\".*\"/VERSION=\"$project_version\"/" install.sh
    else
        echo
    fi

    install_version=$(sed -n 's/\$VERSION *= *"\([^"]*\)"/\1/p' install.ps1)
    echo -n "Install version ('install.ps1'): ${install_version}"
    if [[ "$project_version" != "$install_version" ]]; then
        echo " -> ${project_version}"
        sed -i "s/\$VERSION *= *\".*\"/\$VERSION = \"$project_version\"/" install.ps1
    else
        echo
    fi

    git tag -a "v${project_version}" -m "Arcanist version ${project_version}"
    echo "Tag v${project_version} created, pushing tag to origin..."
    git push origin "v${project_version}"
}
