
pub fn get_css() -> String {
    "<style type=\"text/css\">
            h1 {
                font-size: 14pt;
                margin-left: 15px;
                margin-right: 15px;
                text-align: center;
                font-family: Helvetica, Verdana, Geneva, Arial, sans-serif;
            }
            p {
                font-size: 12pt;
                font-family: Helvetica, Verdana, Geneva, Arial, sans-serif;
                margin-left: 15px;
                margin-right: 15px;
            }
            caption {
                font-family: Helvetica, Verdana, Geneva, Arial, sans-serif;
                font-size: 12pt;
                margin-left: 15px;
                margin-right: 15px;
            }
            table {
                font-size: 12pt;
                font-family: Helvetica, Verdana, Geneva, Arial, sans-serif;
                font-family: arial, sans-serif;
                border-collapse: collapse;
                align: center;
            }
            td, th {
                border: 1px solid #222222;
                text-align: centre;
                padding: 8px;
            }
            tr:nth-child(even) {
                background-color: #dddddd;
            }
            .numberCell {
                text-align: right;
            }
            .header {
                font-weight: bold;
                text-align: center;
            }
        </style>".to_string()
}