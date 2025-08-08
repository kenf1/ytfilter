class GetRequest
{
    public static async Task<string> FetchXmlAsync(string url)
    {
        using HttpClient client = new HttpClient();
        return await client.GetStringAsync(url);
    }
}