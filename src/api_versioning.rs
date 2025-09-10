//! API Versioning module для The Hot Pot Spot
//! 
//! Обеспечивает версионирование API, OpenAPI/Swagger документацию
//! и обратную совместимость для промышленного уровня.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Версия API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ApiVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn from_string(version_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid version format. Expected: major.minor.patch".to_string());
        }

        let major = parts[0].parse::<u32>().map_err(|_| "Invalid major version")?;
        let minor = parts[1].parse::<u32>().map_err(|_| "Invalid minor version")?;
        let patch = parts[2].parse::<u32>().map_err(|_| "Invalid patch version")?;

        Ok(Self { major, minor, patch })
    }

    pub fn is_compatible_with(&self, other: &ApiVersion) -> bool {
        // Major версии должны совпадать для совместимости
        self.major == other.major
    }

    pub fn is_newer_than(&self, other: &ApiVersion) -> bool {
        if self.major != other.major {
            return self.major > other.major;
        }
        if self.minor != other.minor {
            return self.minor > other.minor;
        }
        self.patch > other.patch
    }
}

/// Статус версии API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersionStatus {
    Development,  // В разработке
    Beta,         // Бета версия
    Stable,       // Стабильная версия
    Deprecated,   // Устаревшая версия
    Retired,      // Снята с поддержки
}

/// Информация о версии API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: ApiVersion,
    pub status: VersionStatus,
    pub release_date: DateTime<Utc>,
    pub deprecation_date: Option<DateTime<Utc>>,
    pub retirement_date: Option<DateTime<Utc>>,
    pub changelog: Vec<ChangelogEntry>,
    pub breaking_changes: Vec<String>,
    pub new_features: Vec<String>,
    pub bug_fixes: Vec<String>,
}

/// Запись в changelog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogEntry {
    pub version: ApiVersion,
    pub date: DateTime<Utc>,
    pub changes: Vec<ChangeEntry>,
}

/// Запись об изменении
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEntry {
    pub change_type: ChangeType,
    pub description: String,
    pub impact: ChangeImpact,
    pub affected_endpoints: Vec<String>,
}

/// Тип изменения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    Added,        // Добавлено
    Changed,      // Изменено
    Deprecated,   // Устарело
    Removed,      // Удалено
    Fixed,        // Исправлено
    Security,     // Безопасность
}

/// Влияние изменения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeImpact {
    None,         // Без влияния
    Minor,        // Незначительное
    Major,        // Значительное
    Breaking,     // Критическое (ломает совместимость)
}

/// Конфигурация API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub current_version: ApiVersion,
    pub supported_versions: Vec<ApiVersion>,
    pub default_version: ApiVersion,
    pub version_header: String,
    pub version_query_param: String,
    pub version_path_param: String,
    pub deprecation_warning_days: u32,
    pub retirement_notice_days: u32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            current_version: ApiVersion::new(1, 0, 0),
            supported_versions: vec![ApiVersion::new(1, 0, 0)],
            default_version: ApiVersion::new(1, 0, 0),
            version_header: "API-Version".to_string(),
            version_query_param: "version".to_string(),
            version_path_param: "v".to_string(),
            deprecation_warning_days: 90,
            retirement_notice_days: 30,
        }
    }
}

/// Менеджер версий API
pub struct ApiVersionManager {
    config: ApiConfig,
    versions: HashMap<String, VersionInfo>,
    changelog: Vec<ChangelogEntry>,
}

impl ApiVersionManager {
    /// Создание нового менеджера версий
    pub fn new(config: ApiConfig) -> Self {
        let mut manager = Self {
            config,
            versions: HashMap::new(),
            changelog: Vec::new(),
        };

        // Инициализируем базовую версию
        manager.initialize_base_version();

        manager
    }

    /// Инициализация базовой версии
    fn initialize_base_version(&mut self) {
        let version_info = VersionInfo {
            version: self.config.current_version.clone(),
            status: VersionStatus::Stable,
            release_date: Utc::now(),
            deprecation_date: None,
            retirement_date: None,
            changelog: Vec::new(),
            breaking_changes: vec![
                "Initial API release".to_string(),
                "All endpoints are new".to_string(),
            ],
            new_features: vec![
                "Menu management".to_string(),
                "Token distribution".to_string(),
                "Blockchain integration".to_string(),
                "KYC/AML system".to_string(),
                "HD wallet management".to_string(),
                "Relayer service".to_string(),
                "Database integration".to_string(),
                "Observability system".to_string(),
            ],
            bug_fixes: Vec::new(),
        };

        self.versions.insert(
            self.config.current_version.to_string(),
            version_info,
        );

        // Добавляем в changelog
        let changelog_entry = ChangelogEntry {
            version: self.config.current_version.clone(),
            date: Utc::now(),
            changes: vec![
                ChangeEntry {
                    change_type: ChangeType::Added,
                    description: "Initial API release with full functionality".to_string(),
                    impact: ChangeImpact::None,
                    affected_endpoints: vec!["All endpoints".to_string()],
                },
            ],
        };

        self.changelog.push(changelog_entry);
    }

    /// Получение информации о версии
    pub fn get_version_info(&self, version: &ApiVersion) -> Option<&VersionInfo> {
        self.versions.get(&version.to_string())
    }

    /// Получение текущей версии
    pub fn get_current_version(&self) -> &ApiVersion {
        &self.config.current_version
    }

    /// Получение поддерживаемых версий
    pub fn get_supported_versions(&self) -> &Vec<ApiVersion> {
        &self.config.supported_versions
    }

    /// Проверка поддержки версии
    pub fn is_version_supported(&self, version: &ApiVersion) -> bool {
        self.config.supported_versions.iter().any(|v| v == version)
    }

    /// Проверка совместимости версий
    pub fn are_versions_compatible(&self, version1: &ApiVersion, version2: &ApiVersion) -> bool {
        version1.is_compatible_with(version2)
    }

    /// Получение статуса версии
    pub fn get_version_status(&self, version: &ApiVersion) -> Option<VersionStatus> {
        self.get_version_info(version).map(|info| info.status.clone())
    }

    /// Проверка на устаревшую версию
    pub fn is_version_deprecated(&self, version: &ApiVersion) -> bool {
        if let Some(info) = self.get_version_info(version) {
            matches!(info.status, VersionStatus::Deprecated | VersionStatus::Retired)
        } else {
            true // Неизвестная версия считается устаревшей
        }
    }

    /// Получение предупреждения о версии
    pub fn get_version_warning(&self, version: &ApiVersion) -> Option<VersionWarning> {
        if let Some(info) = self.get_version_info(version) {
            match info.status {
                VersionStatus::Deprecated => {
                    if let Some(deprecation_date) = info.deprecation_date {
                        let days_until_retirement = (info.retirement_date.unwrap_or(Utc::now() + chrono::Duration::days(30)) - Utc::now()).num_days();
                        Some(VersionWarning {
                            warning_type: WarningType::Deprecation,
                            message: format!("API version {} is deprecated and will be retired in {} days", version.to_string(), days_until_retirement),
                            retirement_date: info.retirement_date,
                        })
                    } else {
                        None
                    }
                },
                VersionStatus::Retired => {
                    Some(VersionWarning {
                        warning_type: WarningType::Retirement,
                        message: format!("API version {} has been retired and is no longer supported", version.to_string()),
                        retirement_date: info.retirement_date,
                    })
                },
                _ => None,
            }
        } else {
            Some(VersionWarning {
                warning_type: WarningType::Unsupported,
                message: format!("API version {} is not supported", version.to_string()),
                retirement_date: None,
            })
        }
    }

    /// Получение changelog
    pub fn get_changelog(&self, version: Option<&ApiVersion>) -> Vec<&ChangelogEntry> {
        if let Some(version) = version {
            self.changelog.iter().filter(|entry| &entry.version == version).collect()
        } else {
            self.changelog.iter().collect()
        }
    }

    /// Получение статистики версий
    pub fn get_version_statistics(&self) -> VersionStatistics {
        let mut status_counts = HashMap::new();
        let mut total_endpoints = 0;
        let mut deprecated_endpoints = 0;

        for (_, info) in &self.versions {
            *status_counts.entry(format!("{:?}", info.status)).or_insert(0) += 1;
            total_endpoints += info.new_features.len() + info.bug_fixes.len();
            if matches!(info.status, VersionStatus::Deprecated | VersionStatus::Retired) {
                deprecated_endpoints += info.new_features.len() + info.bug_fixes.len();
            }
        }

        VersionStatistics {
            total_versions: self.versions.len() as u32,
            current_version: self.config.current_version.to_string(),
            supported_versions: self.config.supported_versions.len() as u32,
            deprecated_versions: status_counts.get("Deprecated").copied().unwrap_or(0),
            retired_versions: status_counts.get("Retired").copied().unwrap_or(0),
            total_endpoints: total_endpoints as u32,
            deprecated_endpoints: deprecated_endpoints as u32,
            status_counts,
        }
    }

    /// Генерация OpenAPI спецификации
    pub fn generate_openapi_spec(&self, version: &ApiVersion) -> Result<OpenApiSpec, String> {
        if !self.is_version_supported(version) {
            return Err(format!("Version {} is not supported", version.to_string()));
        }

        let version_info = self.get_version_info(version)
            .ok_or_else(|| format!("Version info not found for {}", version.to_string()))?;

        let spec = OpenApiSpec {
            openapi: "3.0.3".to_string(),
            info: OpenApiInfo {
                title: "The Hot Pot Spot API".to_string(),
                description: format!("API for The Hot Pot Spot blockchain-based restaurant franchise system. Version {}", version.to_string()),
                version: version.to_string(),
                contact: Some(OpenApiContact {
                    name: "The Hot Pot Spot Team".to_string(),
                    email: "api@thehotpotspot.com".to_string(),
                    url: Some("https://thehotpotspot.com".to_string()),
                }),
                license: Some(OpenApiLicense {
                    name: "MIT".to_string(),
                    url: Some("https://opensource.org/licenses/MIT".to_string()),
                }),
            },
            servers: vec![
                OpenApiServer {
                    url: "http://127.0.0.1:8080".to_string(),
                    description: Some("Development server".to_string()),
                },
                OpenApiServer {
                    url: "https://api.thehotpotspot.com".to_string(),
                    description: Some("Production server".to_string()),
                },
            ],
            paths: self.generate_paths(version)?,
            components: Some(OpenApiComponents {
                schemas: self.generate_schemas(version)?,
                security_schemes: Some(self.generate_security_schemes()),
            }),
            security: Some(vec![
                HashMap::from([("ApiKeyAuth".to_string(), Vec::new())])
            ]),
            tags: self.generate_tags(),
        };

        Ok(spec)
    }

    /// Генерация путей API
    fn generate_paths(&self, version: &ApiVersion) -> Result<HashMap<String, OpenApiPathItem>, String> {
        let mut paths = HashMap::new();

        // Базовые пути для версии 1.0.0
        if version.major == 1 {
            paths.insert("/api".to_string(), OpenApiPathItem {
                post: Some(OpenApiOperation {
                    summary: "Main API endpoint".to_string(),
                    description: Some("Main API endpoint for all operations".to_string()),
                    operation_id: Some("main_api".to_string()),
                    tags: Some(vec!["API".to_string()]),
                    request_body: Some(OpenApiRequestBody {
                        description: "API request".to_string(),
                        content: HashMap::from([
                            ("application/json".to_string(), OpenApiMediaType {
                                schema: Some(OpenApiSchema::Reference("#/components/schemas/ApiRequest".to_string())),
                            })
                        ]),
                        required: true,
                    }),
                    responses: HashMap::from([
                        ("200".to_string(), OpenApiResponse {
                            description: "Successful response".to_string(),
                            content: Some(HashMap::from([
                                ("application/json".to_string(), OpenApiMediaType {
                                    schema: Some(OpenApiSchema::Reference("#/components/schemas/ApiResponse".to_string())),
                                })
                            ])),
                        }),
                        ("400".to_string(), OpenApiResponse {
                            description: "Bad request".to_string(),
                            content: Some(HashMap::from([
                                ("application/json".to_string(), OpenApiMediaType {
                                    schema: Some(OpenApiSchema::Reference("#/components/schemas/ErrorResponse".to_string())),
                                })
                            ])),
                        }),
                    ]),
                    security: Some(vec![HashMap::from([("ApiKeyAuth".to_string(), Vec::new())])]),
                }),
            });

            // Добавляем специфичные пути для разных модулей
            paths.insert("/api/v1/menu".to_string(), OpenApiPathItem {
                get: Some(OpenApiOperation {
                    summary: "Get menu".to_string(),
                    description: Some("Get the current menu".to_string()),
                    operation_id: Some("get_menu".to_string()),
                    tags: Some(vec!["Menu".to_string()]),
                    responses: HashMap::from([
                        ("200".to_string(), OpenApiResponse {
                            description: "Menu retrieved successfully".to_string(),
                            content: Some(HashMap::from([
                                ("application/json".to_string(), OpenApiMediaType {
                                    schema: Some(OpenApiSchema::Reference("#/components/schemas/MenuResponse".to_string())),
                                })
                            ])),
                        }),
                    ]),
                }),
            });

            paths.insert("/api/v1/kyc".to_string(), OpenApiPathItem {
                post: Some(OpenApiOperation {
                    summary: "KYC operations".to_string(),
                    description: Some("Perform KYC/AML operations".to_string()),
                    operation_id: Some("kyc_operations".to_string()),
                    tags: Some(vec!["KYC/AML".to_string()]),
                    request_body: Some(OpenApiRequestBody {
                        description: "KYC request".to_string(),
                        content: HashMap::from([
                            ("application/json".to_string(), OpenApiMediaType {
                                schema: Some(OpenApiSchema::Reference("#/components/schemas/KYCRequest".to_string())),
                            })
                        ]),
                        required: true,
                    }),
                    responses: HashMap::from([
                        ("200".to_string(), OpenApiResponse {
                            description: "KYC operation completed".to_string(),
                            content: Some(HashMap::from([
                                ("application/json".to_string(), OpenApiMediaType {
                                    schema: Some(OpenApiSchema::Reference("#/components/schemas/KYCResponse".to_string())),
                                })
                            ])),
                        }),
                    ]),
                }),
            });
        }

        Ok(paths)
    }

    /// Генерация схем
    fn generate_schemas(&self, version: &ApiVersion) -> Result<HashMap<String, OpenApiSchema>, String> {
        let mut schemas = HashMap::new();

        // Базовые схемы
        schemas.insert("ApiRequest".to_string(), OpenApiSchema::Object(OpenApiSchemaObject {
            schema_type: Some("object".to_string()),
            description: Some("API request structure".to_string()),
            properties: Some(HashMap::from([
                ("request_type".to_string(), OpenApiSchema::String(OpenApiSchemaString {
                    description: Some("Type of request".to_string()),
                    enum_values: Some(vec![
                        "GetMenu".to_string(),
                        "AddMenuItem".to_string(),
                        "ProcessPurchase".to_string(),
                        "RegisterUser".to_string(),
                        "StartKYCProcess".to_string(),
                    ]),
                })),
                ("data".to_string(), OpenApiSchema::Object(OpenApiSchemaObject {
                    schema_type: Some("object".to_string()),
                    description: Some("Request data".to_string()),
                })),
            ])),
            required: Some(vec!["request_type".to_string()]),
        }));

        schemas.insert("ApiResponse".to_string(), OpenApiSchema::Object(OpenApiSchemaObject {
            schema_type: Some("object".to_string()),
            description: Some("API response structure".to_string()),
            properties: Some(HashMap::from([
                ("success".to_string(), OpenApiSchema::Boolean(OpenApiSchemaBoolean {
                    description: Some("Whether the request was successful".to_string()),
                })),
                ("data".to_string(), OpenApiSchema::Object(OpenApiSchemaObject {
                    schema_type: Some("object".to_string()),
                    description: Some("Response data".to_string()),
                })),
                ("error".to_string(), OpenApiSchema::String(OpenApiSchemaString {
                    description: Some("Error message if any".to_string()),
                })),
            ])),
        }));

        schemas.insert("ErrorResponse".to_string(), OpenApiSchema::Object(OpenApiSchemaObject {
            schema_type: Some("object".to_string()),
            description: Some("Error response structure".to_string()),
            properties: Some(HashMap::from([
                ("error".to_string(), OpenApiSchema::String(OpenApiSchemaString {
                    description: Some("Error message".to_string()),
                })),
                ("code".to_string(), OpenApiSchema::String(OpenApiSchemaString {
                    description: Some("Error code".to_string()),
                })),
                ("timestamp".to_string(), OpenApiSchema::String(OpenApiSchemaString {
                    description: Some("Error timestamp".to_string()),
                })),
            ])),
            required: Some(vec!["error".to_string()]),
        }));

        Ok(schemas)
    }

    /// Генерация схем безопасности
    fn generate_security_schemes(&self) -> HashMap<String, OpenApiSecurityScheme> {
        HashMap::from([
            ("ApiKeyAuth".to_string(), OpenApiSecurityScheme {
                scheme_type: "apiKey".to_string(),
                name: "X-API-Key".to_string(),
                location: "header".to_string(),
                description: Some("API key for authentication".to_string()),
            }),
        ])
    }

    /// Генерация тегов
    fn generate_tags(&self) -> Vec<OpenApiTag> {
        vec![
            OpenApiTag {
                name: "API".to_string(),
                description: Some("Main API operations".to_string()),
            },
            OpenApiTag {
                name: "Menu".to_string(),
                description: Some("Menu management operations".to_string()),
            },
            OpenApiTag {
                name: "KYC/AML".to_string(),
                description: Some("KYC/AML operations".to_string()),
            },
            OpenApiTag {
                name: "Wallet".to_string(),
                description: Some("Wallet management operations".to_string()),
            },
            OpenApiTag {
                name: "Database".to_string(),
                description: Some("Database operations".to_string()),
            },
            OpenApiTag {
                name: "Observability".to_string(),
                description: Some("Observability operations".to_string()),
            },
        ]
    }
}

/// Предупреждение о версии
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionWarning {
    pub warning_type: WarningType,
    pub message: String,
    pub retirement_date: Option<DateTime<Utc>>,
}

/// Тип предупреждения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WarningType {
    Deprecation,
    Retirement,
    Unsupported,
}

/// Статистика версий
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionStatistics {
    pub total_versions: u32,
    pub current_version: String,
    pub supported_versions: u32,
    pub deprecated_versions: u32,
    pub retired_versions: u32,
    pub total_endpoints: u32,
    pub deprecated_endpoints: u32,
    pub status_counts: HashMap<String, u32>,
}

// OpenAPI структуры
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: OpenApiInfo,
    pub servers: Vec<OpenApiServer>,
    pub paths: HashMap<String, OpenApiPathItem>,
    pub components: Option<OpenApiComponents>,
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
    pub tags: Vec<OpenApiTag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiInfo {
    pub title: String,
    pub description: String,
    pub version: String,
    pub contact: Option<OpenApiContact>,
    pub license: Option<OpenApiLicense>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiContact {
    pub name: String,
    pub email: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiLicense {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiServer {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiPathItem {
    pub get: Option<OpenApiOperation>,
    pub post: Option<OpenApiOperation>,
    pub put: Option<OpenApiOperation>,
    pub delete: Option<OpenApiOperation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiOperation {
    pub summary: String,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub request_body: Option<OpenApiRequestBody>,
    pub responses: HashMap<String, OpenApiResponse>,
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiRequestBody {
    pub description: String,
    pub content: HashMap<String, OpenApiMediaType>,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiResponse {
    pub description: String,
    pub content: Option<HashMap<String, OpenApiMediaType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiMediaType {
    pub schema: Option<OpenApiSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum OpenApiSchema {
    String(OpenApiSchemaString),
    Number(OpenApiSchemaNumber),
    Boolean(OpenApiSchemaBoolean),
    Object(OpenApiSchemaObject),
    Array(OpenApiSchemaArray),
    Reference(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiSchemaString {
    pub description: Option<String>,
    pub enum_values: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiSchemaNumber {
    pub description: Option<String>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiSchemaBoolean {
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiSchemaObject {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, OpenApiSchema>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiSchemaArray {
    pub description: Option<String>,
    pub items: Option<Box<OpenApiSchema>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiComponents {
    pub schemas: HashMap<String, OpenApiSchema>,
    pub security_schemes: Option<HashMap<String, OpenApiSecurityScheme>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiSecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: String,
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiTag {
    pub name: String,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_version_parsing() {
        let version = ApiVersion::from_string("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_api_version_compatibility() {
        let v1_0_0 = ApiVersion::new(1, 0, 0);
        let v1_1_0 = ApiVersion::new(1, 1, 0);
        let v2_0_0 = ApiVersion::new(2, 0, 0);

        assert!(v1_0_0.is_compatible_with(&v1_1_0));
        assert!(!v1_0_0.is_compatible_with(&v2_0_0));
    }

    #[test]
    fn test_version_manager() {
        let config = ApiConfig::default();
        let manager = ApiVersionManager::new(config);
        
        assert_eq!(manager.get_current_version().to_string(), "1.0.0");
        assert!(manager.is_version_supported(&ApiVersion::new(1, 0, 0)));
    }

    #[test]
    fn test_openapi_generation() {
        let config = ApiConfig::default();
        let manager = ApiVersionManager::new(config);
        let version = ApiVersion::new(1, 0, 0);
        
        let spec = manager.generate_openapi_spec(&version).unwrap();
        assert_eq!(spec.info.title, "The Hot Pot Spot API");
        assert_eq!(spec.info.version, "1.0.0");
    }
}

