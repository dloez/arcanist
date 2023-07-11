#!/bin/bash

release() {
    command -v jq &> "/dev/null"
    if [[ $? -gt 0 ]]; then
        echo "Missing required tool 'jq'"
        exit
    fi

    project_version=$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name == "arcanist").version')
    echo "Project version ('Cargo.toml') : ${project_version}"

    commit=false
    install_version=$(sed -n "s/VERSION=\"\([^\"]*\)\"/\1/p" install.sh)
    echo -n "Install version ('install.sh') : ${install_version}"
    if [[ "$project_version" != "$install_version" ]]; then
        echo " -> ${project_version}"
        sed -i "s/VERSION=\".*\"/VERSION=\"$project_version\"/" install.sh
        commit=true
    else
        echo
    fi

    install_version=$(sed -n 's/\$VERSION *= *"\([^"]*\)"/\1/p' install.ps1)
    echo -n "Install version ('install.ps1'): ${install_version}"
    if [[ "$project_version" != "$install_version" ]]; then
        echo " -> ${project_version}"
        sed -i "s/\$VERSION *= *\".*\"/\$VERSION = \"$project_version\"/" install.ps1
        commit=true
    else
        echo
    fi

    if [[ $commit == true ]]; then
        git add install.sh install.ps1
        git commit -m 'Bump install versions'
        echo "Commit with bumped install version created, commit ready to be pushed, press any key to continue..."
        read -d'' -s -n1
        git push
    fi

    git tag -a "v${project_version}" -m "Arcanist version ${project_version}"
    if [[ $? -gt 0 ]]; then
        echo "Failed to create tag 'v${project_version}'"
        exit
    fi
    echo "Tag v${project_version} created, pushing tag to origin..."
    echo "Ready to push tag v${project_version}, press any key to continue"
    read -d'' -s -n1
    git push origin "v${project_version}"
}
