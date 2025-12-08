function maker(content) {
    const allLines = content.split("\r\n").filter(v=>v.length > 0);
    return allLines.map(v => {
        const items = v.split(',');
        const fileName = `@/compose-${items[0]}.txt`
        return {
            [fileName]: `${items[1]}`
        }
    })
}

maker.compose = true;