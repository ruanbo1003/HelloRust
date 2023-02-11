
pub fn get_type_name<T>(_: T) -> &'static str {
    return std::any::type_name::<T>();
}
