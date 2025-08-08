public class MediaGroupInfo
{
    public string MediaTitle { get; set; } = "N/A";
    public string MediaContentUrl { get; set; } = "N/A";
    public string MediaDescription { get; set; } = "N/A";
    public MediaCommunityInfo Community { get; set; } = new MediaCommunityInfo();
}
