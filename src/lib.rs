pub mod csv_helper;
pub mod sql_builder;

#[cfg(test)]
mod tests {
    use crate::sql_builder::build_sql_statement;

    #[test]
    fn build_sql_statement_returns_statement(){
        let statement = build_sql_statement(vec!["RunDt".to_string(), "Amount".to_string(), "Some Header With Spaces".to_string()], "default_table", "default_db");

        let expected: String = r#"USE [default_db]
GO

IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA='dbo' AND TABLE_NAME = 'default_table')
BEGIN
CREATE TABLE [dbo].[default_table]
[RowID] [int] IDENTITY(1,1) NOT NULL,
[RunDt] [date] NULL,
[Amount] [numeric](18,2) NULL,
[SomeHeaderWithSpaces] [varchar](50) NULL,
[CreateDate] [date] NOT NULL
CONSTRAINT [PK_default_table] PRIMARY KEY CLUSTERED(
[RowID] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]
ALTER TABLE [dbo].[default_table] ADD CONSTRAINT [DF_default_table_CreateDate] DEFAULT (getdate()) FOR [CreateDate]
END
GO"#.to_string();

        assert_eq!(statement, expected)

    }
}
