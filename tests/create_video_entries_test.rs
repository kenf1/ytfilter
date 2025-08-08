#[cfg(test)]
mod tests {
    use ytfilter::{
        data_layer::response::create_video_entries,
        models::{
            media::{
                community::MediaCommunity, content::MediaContent,
                group::MediaGroup, star_rating::MediaStarRating,
                statistics::MediaStatistics,
            },
            response::{entry::Entry, feed::Feed},
            video_entry::VideoEntry,
        },
    };

    #[test]
    fn test_create_video_entries() {
        let feed = Feed {
            entry: vec![
                Entry {
                    video_id: "vid123".to_string(),
                    published: "2024-08-07T10:00:00Z".to_string(),
                    updated: "2024-08-07T12:00:00Z".to_string(),
                    media_group: MediaGroup {
                        title: "Test Video 1".to_string(),
                        content: MediaContent {
                            url: "http://example.com/video1.mp4".to_string(),
                            content_type: Some("image".to_string()),
                            height: Some(400),
                            width: Some(400),
                        },
                        description: "Description 1".to_string(),
                        community: MediaCommunity {
                            star_rating: MediaStarRating {
                                count: 100,
                                average: 4.5,
                                min: 1,
                                max: 5,
                            },
                            statistics: MediaStatistics { views: 1000 },
                        },
                    },
                },
                Entry {
                    video_id: "vid456".to_string(),
                    published: "2024-08-06T09:00:00Z".to_string(),
                    updated: "2024-08-06T09:10:00Z".to_string(),
                    media_group: MediaGroup {
                        title: "Test Video 2".to_string(),
                        content: MediaContent {
                            url: "http://example.com/video2.mp4".to_string(),
                            content_type: None,
                            height: None,
                            width: None,
                        },
                        description: "Description 2".to_string(),
                        community: MediaCommunity {
                            star_rating: MediaStarRating {
                                count: 200,
                                average: 3.8,
                                min: 1,
                                max: 5,
                            },
                            statistics: MediaStatistics { views: 2000 },
                        },
                    },
                },
            ],
        };

        let videos = create_video_entries(feed);

        let expected = vec![
            VideoEntry {
                video_id: "vid123".to_string(),
                published: "2024-08-07T10:00:00Z".to_string(),
                updated: "2024-08-07T12:00:00Z".to_string(),
                title: "Test Video 1".to_string(),
                content_url: "http://example.com/video1.mp4".to_string(),
                description: "Description 1".to_string(),
                star_rating_count: 100,
                star_rating_average: 4.5,
                star_rating_min: 1,
                star_rating_max: 5,
                views: 1000,
            },
            VideoEntry {
                video_id: "vid456".to_string(),
                published: "2024-08-06T09:00:00Z".to_string(),
                updated: "2024-08-06T09:10:00Z".to_string(),
                title: "Test Video 2".to_string(),
                content_url: "http://example.com/video2.mp4".to_string(),
                description: "Description 2".to_string(),
                star_rating_count: 200,
                star_rating_average: 3.8,
                star_rating_min: 1,
                star_rating_max: 5,
                views: 2000,
            },
        ];

        assert_eq!(videos, expected);
    }
}
