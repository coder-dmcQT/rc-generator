function generate_domain_file(item, packageName) {
    const headerContent = [
        `package ${packageName}.domain;`,
        `import jakarta.persistence.*;`,
        `import lombok.AllArgsConstructor;`,
        `import lombok.Builder;`,
        `import lombok.Data;`,
        `import lombok.NoArgsConstructor;`,
    ]

    const classAnnotations = [
        `@Entity`,
        `@Table(name = "${item.table_name}")`,
        `@Data`,
        `@Builder`,
        `@NoArgsConstructor`,
        `@AllArgsConstructor`
    ]
    const classContent = [
        `public class ${item.name} {`
    ]

    for (const key in item.fields) {
        if (key === 'id') {
            classContent.push(
                `@Id`, `@GeneratedValue(strategy = GenerationType.IDENTITY)`,
                `private ${item.fields[key].type} ${key};`
            )
        } else {
            const columnProperties = [];
            const targetItem = item.fields[key];
            if (targetItem.length !== undefined) {
                columnProperties.push(`length = ${targetItem.length}`)
            }
            if (targetItem.nullable !== undefined) {
                columnProperties.push(`nullable = ${targetItem.nullable}`)
            }
            if (targetItem.unique !== undefined) {
                columnProperties.push(`unique = ${targetItem.unique}`)
            }
            classContent.push(
                columnProperties.length > 0 ? `@Column(${columnProperties.join(', ')})` : `@Column`,
                `private ${item.fields[key].type} ${key};`
            )
        }
    }

    classContent.push('}')
    const output = [...headerContent, ...classAnnotations, ...classContent]

    return output.join('\r\n');
}

function generate_repository_files(item, packageName, typeItem) {
    const typeName = typeItem.name
    const idType = typeItem.fields.id.type;
    const header_content = [
        `package ${packageName}.repository;`,
        `import java.util.List;`,
        `import java.util.Optional;`,
        `import org.springframework.data.repository.Repository;`,
        `import ${packageName}.domain.${typeName};`
    ]

    const class_content = [
        `public interface ${item.name} extends Repository<${typeName}, ${idType}>{`
    ]
    const fields = typeItem.fields;
    const allKeys = Object.keys(fields).map(item => {
        return {
            re: new RegExp(item, 'ig'),
            item
        }
    });
    for (const methodName of item.methods) {
        const keys_mapped = allKeys.filter(v => {
            return methodName.match(v.re)
        });
        if (keys_mapped.length) {
            if (methodName.startsWith("find")) {
                // If any unique field is included, then typed as Options, or else List
                const mapped_fields = keys_mapped.map(item => {
                    return {
                        name: item.item,
                        type: fields[item.item].type,
                        unique: Boolean(fields[item.item].unique) || item.item === 'id'
                    }
                })
                const isAnyOneUnique = mapped_fields.some(v => v.unique);
                if (isAnyOneUnique) {
                    class_content.push(`Optional<${typeName}> ${methodName}(${mapped_fields.map(a => `${a.type} ${a.name}`).join(', ')});`)
                } else {
                    class_content.push(`List<${typeName}> ${methodName}(${mapped_fields.map(a => `${a.type} ${a.name}`).join(', ')});`)
                }
            } else if (methodName.startsWith("exists")) {
                const mapped_fields = keys_mapped.map(item => {
                    return {
                        name: item.item,
                        type: fields[item.item].type
                    }
                })
                class_content.push(`boolean ${methodName}(${mapped_fields.map(a => `${a.type} ${a.name}`).join(', ')});`)
            }
        }
    }

    class_content.push(`}`)
    return header_content.concat(class_content).join('\r\n');
}

function convert_display(value) {
    if (typeof value === 'string') {
        return `"${value}"`
    }
    return value;
}

function generate_dto_request(item, packageName) {
    const headerContent = [
        `package ${packageName}.dto.request;`,
        `import lombok.Data;`
    ]

    const importMapping = {
        Size: "import jakarta.validation.constraints.Size;",
        Email: "import jakarta.validation.constraints.Email;",
        NotBlank: "import jakarta.validation.constraints.NotBlank;"
    }

    const classContent = [
        `@Data`,
        `public class ${item.name} {`
    ];
    const fields = item.fields;

    for (const field in fields) {
        const fieldConfig = fields[field];
        const fieldCodes = [
            `private ${fieldConfig.type} ${field};`
        ]
        for (const config_key in fieldConfig) {
            if (config_key !== 'type' && config_key in importMapping) {
                if (importMapping[config_key]) {
                    headerContent.push(importMapping[config_key])
                    importMapping[config_key] = ''
                }
                const notation_configs = Object.entries(
                    fieldConfig[config_key]
                );
                fieldCodes.unshift(
                    `@${config_key}(${notation_configs.map(item => `${item[0]} = ${convert_display(item[1])}`).join(', ')})`
                )
            }
        }
        classContent.push(...fieldCodes)
    }
    classContent.push('}')

    return headerContent.concat(classContent).join('\r\n')
}

function generate_dto_res(item, packageName) {
    const headerContent = [
        `package ${packageName}.dto.response;`,
        `import lombok.Builder;`,
        `import lombok.Data;`
    ]

    const classContent = [
        `@Data`,
        `@Builder`,
        `public class ${item.name} {`
    ]

    for (const field_key in item.fields) {
        classContent.push(`private ${item.fields[field_key]} ${field_key};`)
    }
    classContent.push('}')
    return headerContent.concat(classContent).join('\r\n')
}

function make_api_response(package_prefix) {
    const header = [
        `package ${package_prefix}.dto.api;`,
        `import lombok.AllArgsConstructor;`,
        `import lombok.Builder;`,
        `import lombok.Data;`,
        `import lombok.NoArgsConstructor;`
    ]
    const classContent = [
        `@Data`,
        `@Builder`,
        `@NoArgsConstructor`,
        `@AllArgsConstructor`,
        `public class ApiResponse<T> {`,
        `private init code;`,
        `private String message;`,
        `private T data;`,
        `public static <T> ApiResponse<T> success(T data){`,
        `return ApiResponse.<T>builder()`,
        `.code(200)`,
        `.message("Operation success!")`,
        `.data(data)`,
        `.build();`,
        `}`,
        `public static <T> ApiResponse<T> error(int code, String message){`,
        `return ApiResponse.<T>builder()`,
        `.code(code)`,
        `.message(message)`,
        `.data(null)`,
        `.build();`,
        `}`,
        '}'
    ]
    return header.concat(classContent).join('\r\n')
}

function camelizeCase(word) {
    return word[0].toUpperCase().concat(word.slice(1))
}

function generate_service_impl(item, packageName, allSymbols) {
    // I just simply include all symbols here, you can define your own search and include logic
    const interface_header = [
        `package ${packageName}.service;`,
        ...allSymbols.map(symbol => `import ${symbol};`)
    ];
    const interface_content = [
        `public interface ${item.name} {`
    ]

    const impl_header = [
        `package ${packageName}.service.impl;`,
        `import lombok.RequiredArgsConstructor;`,
        `import org.springframework.stereotype.Service;`,
        `import org.springframework.transaction.annotation.Transactional;`,
        `import ${packageName}.service.${item.name};`,
        ...allSymbols.map(symbol => `import ${symbol};`)
    ]

    const impl_content = [
        `public class ${item.name}Impl implements ${item.name} {`,
    ]

    for (const method in item.methods) {
        const method_config = item.methods[method];
        const method_for_interface = [
            `${method_config.returnTyping} ${method}(${method_config.paramsTyping.join(', ')});`
        ]

        const method_for_impl = [
            `public ${method_config.returnTyping} ${method}(${method_config.paramsTyping.join(', ')}) {`
        ]
        if (method_config.Transactional) {
            const transactionalEntries = Object.entries(method_config.Transactional);
            method_for_impl.unshift('@Transactional')
            method_for_impl.unshift(`@Override`)
        } else {
            method_for_impl.unshift(`@Override`)
        }
        const result_return = camelizeCase(method_config.returnTyping)
        method_for_impl.push(`${method_config.returnTyping} ${result_return} = null;`)
        method_for_impl.push(`return ${result_return};`)

        method_for_impl.push('}');

        interface_content.push(...method_for_interface)
        impl_content.push(...method_for_impl)
    }

    interface_content.push('}')
    impl_content.push('}')

    return [
        interface_header.concat(interface_content).join('\r\n'),
        impl_header.concat(impl_content).join('\r\n')
    ]
}

function iterate_for_all_symbols(item, parent_keys) {
    const results = []
    if (Array.isArray(item) && item.every(a => a.name !== undefined)) {
        const prefix = parent_keys.join('.')
        results.push(...item.map(v => `${prefix}.${v.name}`))
    }
    if (typeof item === 'object') {
        for (const key in item) {
            results.push(...iterate_for_all_symbols(item[key], [...parent_keys, key]))
        }
    }
    return results;
}

function toUpperCamelCase(lowerCamelStr) {
    if (typeof lowerCamelStr !== 'string' || lowerCamelStr.trim() === '') {
        return '';
    }
    return lowerCamelStr.replace(/^[a-z]/, (match) => match.toUpperCase());
}

function convert_shorten_to_std(shorten_json) {
    const newlyObject = {
        Prefix: shorten_json.Prefix,
        domain: shorten_json.domain,
        repository: [],
        dto: {
            request: [],
            response: []
        },
        Service: []
    }
    // make repository
    newlyObject.repository = shorten_json.domain.map(function (item) {
        const name = `${item.name}Repository`
        const {fields} = item;
        const allUniqueFields = Object.entries(fields).filter(kv => kv[1].unique);
        const methods = allUniqueFields.map(function (kv) {
            const fieldNameAllCapitalize = toUpperCamelCase(kv[0])
            return [
                `findBy${fieldNameAllCapitalize}`,
                `existsBy${fieldNameAllCapitalize}`
            ]
        }).flat()
        return {
            name,
            methods
        }
    })

    newlyObject.dto.request = shorten_json.domain.map(function (item) {
        const name = `${item.name}Request`
        const fields = Object.entries(item.fields).reduce((q, w) => {
            return {...q, [w[0]]: {type: w[1].type, ...w[1].validation}};
        }, {});
        return {
            name,
            fields
        }
    })
    newlyObject.dto.response = shorten_json.domain.map(function (item) {
        const name = `${item.name}Response`
        const fields = Object.entries(item.fields).reduce(function (q, w) {
            return {
                ...q,
                [w[0]]: w[1].type
            }
        }, {})
        return {
            name, fields
        }
    })
    newlyObject.Service = shorten_json.domain.map(function (item) {
        const name = item.name + 'Service';
        const methods = {
            [`create${item.name}`]: {
                Transactional: {},
                paramsTyping: [`${item.name}Request ${item.name[0]}r`],
                returnTyping: `${item.name}Response`
            },
            ["getAll" + item.name + 's']: {
                "Transactional": {
                    "readOnly": true
                },
                "paramsTyping": [],
                "returnTyping": `List<${item.name}Response>`
            },
            [`get${item.name}ById`]: {
                "Transactional": {
                    "readOnly": true
                },
                "paramsTyping": ["Long id"],
                "returnTyping": `${item.name}Response`
            },
            [`update${item.name}`]: {
                "Transactional": {},
                "paramsTyping": ["Long id", `${item.name}Request ${item.name[0]}r`],
                "returnTyping": `${item.name}Response`
            },
            ["delete" + item.name]: {
                "Transactional": {},
                "paramsTyping": ["Long id"],
                "returnTyping": "void"
            }
        }
        return {
            name, methods
        }
    })

    return newlyObject
}

function toSnake(str) {
    if (!str) {
        return ''
    }
    return str.match(/([A-Z][a-z]+)/g).map(v => v.toLowerCase()).join('_')
}

function generate_controller(domainItem, packagePrefix) {
    const header_content = [
        `package ${packagePrefix}.controller;`,
        `import ${packagePrefix}.dto.api.ApiResponse;`,
        `import ${packagePrefix}.dto.response.${domainItem.name}Response;`,
        `import ${packagePrefix}.dto.request,${domainItem.name}Request;`,
        `import ${packagePrefix}.service.${domainItem.name}Service;`,
        `import jakarta.validation.Valid;`,
        `import lombok.RequiredArgsConstructor;`,
        `import lombok.extern.slf4j.Slf4j;`,
        `import org.springframework.http.HttpStatus;`,
        `import org.springframework.web.bind.annotation.*;`,
        `import java.util.List;`
    ];

    const classContent = [
        `@Slf4j`, `@RestController`, `@RequestMapping("/api/v1/${toSnake(domainItem.name)}")`, `@RequiredArgsConstructor`,
        `public class ${domainItem.name}Controller{`,
        `private final ${domainItem.name}Service ${camelizeCase(`${domainItem.name}Service`)};`,
        `@PostMapping`, `@ResponseStatus(HttpStatus.CREATED)`,
        `public ApiResponse<${domainItem.name}Response> create${domainItem.name}(@Valid @RequestBody ${domainItem.name}Request request) {`,
        `log.info("Initiating ${domainItem.name} for ID {}", request.get${domainItem.name}Id());`,
        `${domainItem.name}Response created${domainItem.name} = ${camelizeCase(`${domainItem.name}Service;`)}`,
        `return ApiResponse.success(${domainItem.name} created successfully, created${domainItem.name});`, '}',
        `@GetMapping`, `@ResponseStatus(HttpStatus.OK)`, `public ApiResponse<List<${domainItem.name}Response>> getAll${domainItem.name}(){`,
        `log.info("Retrieving all ${domainItem.name}s");`, `List<${domainItem.name}Response> resp = ${camelizeCase(`${domainItem.name}Service`)}.getAll${domainItem.name}s;`,
        `return ApiResponse.success(resp);`, `}`,
        `@GetMapping("/{id}")`, `@ResponseStatus(HttpStatus.OK)`,
        `public ApiResponse<${domainItem.name}Response> get${domainItem.name}ById(@PathVariable Long id){`,
        `log.info("Retrieving ${domainItem.name} by id");`, `${domainItem.name}Response resp = ${camelizeCase(`${domainItem.name}Service`)}.get${domainItem.name}ById(id);`,
        `return ApiResponse.success(resp);`, `}`, `@PutMapping("/id")`, `@ResponseStatus(HttpStatus.OK)`,
        `public ApiResponse<${domainItem.name}Response> update${domainItem.name}(@PathVariable Long id, @Valid @RequestBody ${domainItem.name}Request request) {`,
        `log.info("Updating ${domainItem.name} by id");`,
        `${domainItem.name}Response resp = ${camelizeCase(`${domainItem.name}Service`)}.update${domainItem.name}(id, request);`,
        `return ApiResponse.success(resp);`, '}',
        `@DeleteMapping("/{id}")`, `@ResponseStatus(HttpStatus.OK)`,
        `public ApiResponse<void> delete${domainItem.name}(@PathVariable Long id){`,
        `${camelizeCase(`${domainItem.name}Service`)}.delete${domainItem.name}(id);`,
        `return ApiResponse.success(null);`, '}'
    ]

    classContent.push('}')
    return header_content.concat(classContent).join('\r\n')
}

function generate(json_content) {
    const json = convert_shorten_to_std(JSON.parse(json_content))
    const package_prefix = json.Prefix;
    const allSymbols = iterate_for_all_symbols(json, []).map(v => `${package_prefix}.${v}`)

    const results = []

    for (const item of json.domain) {
        const path = `@/java/${package_prefix.replaceAll('.', '/')}/domain/${item.name}.java`;
        const content = generate_domain_file(item, package_prefix);
        const obj = {
            [path]: content
        }
        results.push(obj)
        const controllerPath = `@/java/${package_prefix.replaceAll('.', '/')}/controller/${item.name}Controller.java`
        const contentController = generate_controller(item, package_prefix);
        results.push({
            [controllerPath]: contentController
        })
    }

    // Generate repositories
    for (let i = 0; i < json.repository.length; i++) {
        const item = json.repository[i];
        const domainItem = json.domain[i]
        const path = `@/java/${package_prefix.replaceAll('.', '/')}/repository/${item.name}.java`;
        const content = generate_repository_files(item, package_prefix, domainItem);
        results.push({
            [path]: content
        })
    }

    for (let i = 0; i < json.dto.request.length; i++) {
        const requestItem = json.dto.request[i];
        const path = `@/java/${package_prefix.replaceAll('.', '/')}/dto/request/${requestItem.name}.java`
        const content = generate_dto_request(requestItem, package_prefix)
        results.push({
            [path]: content
        })
    }

    for (let i = 0; i < json.dto.response.length; i++) {
        const item = json.dto.response[i];
        const path = `@/java/${package_prefix.replaceAll('.', '/')}/dto/response/${item.name}.java`
        results.push({
            [path]: generate_dto_res(item, package_prefix)
        })
    }

    for (let i = 0; i < json.Service.length; i++) {
        const item = json.Service[i];
        const files = generate_service_impl(item, package_prefix, allSymbols);
        const path_interface = `@/java/${package_prefix.replaceAll('.', '/')}/service/${item.name}.java`
        const path_impl = `@/java/${package_prefix.replaceAll('.', '/')}/service/impl/${item.name}.java`
        results.push({
            [path_interface]: files[0],
            [path_impl]: files[1]
        })
    }

    const apiResponseClass = make_api_response(package_prefix);

    results.push({
        [`@/java/${package_prefix.replaceAll('.', '/')}/dto/api/ApiResponse.java`]: apiResponseClass
    });


    return results
}

generate.compose = true;