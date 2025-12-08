function maker(content) {
    const allLines = content.split("\r\n").filter(v=>v.length > 0);
    return allLines.map(v => {
        const items = v.split(',');
        const fileName = `@/compose-${items[0]}.txt`
        // this will work: return {[fileName]: `${items[1]}`};
        return [fileName, `${items[1]}`]
    })
    // you can directly return path-content pairs
    // return allLines.reduce((q,w)=>({...q, [[your operation]]}), {})
}

maker.compose = true;