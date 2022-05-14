pub fn build_sql_statement(headers: Vec<String>, table: &str, database: &str) -> String{
    let mut columns: Vec<String> = Vec::with_capacity(headers.len()+2);
    columns.push("[RowID] [int] IDENTITY(1,1) NOT NULL".to_string());

    for header in headers {
        let trimmed_header: String = header.chars().filter(|c| !c.is_whitespace()).collect();
        if trimmed_header.ends_with("Dt") || trimmed_header.ends_with("Date"){
            columns.push(format!("[{}] [date] NULL", trimmed_header));
        } else if trimmed_header.contains("Amt") || trimmed_header.contains("Amount"){
            columns.push(format!("[{}] [numeric](18,2) NULL", trimmed_header))
        }else{
            columns.push(format!("[{}] [varchar](50) NULL", trimmed_header))
        }
    }
    columns.push("[CreateDate] [date] NOT NULL".to_string());

    let result: String = format!(r#"USE [{db}]
GO

IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA='dbo' AND TABLE_NAME = '{table_name}')
BEGIN
CREATE TABLE [dbo].[{table_name}]
{cols}
CONSTRAINT [PK_{table_name}] PRIMARY KEY CLUSTERED(
[RowID] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]
ALTER TABLE [dbo].[{table_name}] ADD CONSTRAINT [DF_{table_name}_CreateDate] DEFAULT (getdate()) FOR [CreateDate]
END
GO"#, db = database, table_name = table, cols = columns.join(",\n"));

    result
}
