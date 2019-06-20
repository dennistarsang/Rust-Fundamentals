mod greetings
{
	mod english
	{
		fn hello() -> String { "hello".to_string() }
		fn goodbye() -> String {"goodbye".to_string()}
	}

	mod french
	{
		fn hello() -> String { "bonjour".to_string() }
		fn goodbye() -> String {"au revoir".to_string()}
	}	
	

}