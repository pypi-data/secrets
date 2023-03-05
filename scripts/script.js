const fs = require('fs');

module.exports = async ({github, fetch}) => {
    let response = await github.rest.repos.listForOrg({
        org: "pypi-data",
        sort: "full_name",
    });

    let repo_names = response.data.map(r => r.full_name).filter(name => name.startsWith("pypi-data/pypi-code-")).map(name => name.split('/')[1]);

    let indexes = [];
    for (const idx in repo_names) {
        let name = repo_names[idx];
        let api_response = await github.rest.repos.getContent({
            owner: "pypi-data",
            repo: name,
            path: "index.json",
        });
        let response = await fetch(api_response.data.download_url);
        let content = await response.json();
        console.log(name);
        // console.log(content);
        let count = Object.values(content.entries).reduce((a, v) => a + v.length, 0);
        let output = {
            url: content.url,
            earliest_release: content.earliest_release,
            latest_release: content.latest_release,
            count
        }
        fs.writeFileSync(`partitions/${name}.json`, JSON.stringify(output));
        console.log(output);
    }
}
